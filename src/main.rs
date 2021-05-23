use std::env;
use std::error::Error;
use std::io;
use std::net::{IpAddr, Ipv6Addr, SocketAddr, TcpStream, ToSocketAddrs};
use std::process;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{App, AppSettings, Arg, SubCommand};
use otpauth;

fn build_ip(base: &str, code: u32) -> Result<String, &'static str> {
    let code_str = format!("{:06}", code);
    if !code_str.is_ascii() {
        return Err("unexpected non-ascii characters while converting code to string");
    }

    let code_chars = code_str.as_bytes();
    if code_chars.len() != 6 {
        return Err("unexpected code len");
    }

    let mut new_ip = String::default();
    let mut idx = 0;

    for chr in base.chars() {
        new_ip.push(if chr == 'x' {
            if idx > 5 {
                return Err("too many placeholder characters");
            }

            let replacement: char = code_chars[idx].into();
            idx += 1;
            replacement
        } else {
            chr
        });
    }

    if idx != 6 {
        return Err("did not replace totp properly");
    }

    Ok(new_ip)
}

fn main() {
    let app = App::new("tosh")
        .version("0.0.0")
        .author("Mark V. <mikroskeem@mikroskeem.eu>")
        .about("Generates IPv6 address based on TOTP for ssh client/server")
        .subcommand(SubCommand::with_name("generate").about("Generates an IPv6 address"))
        .subcommand(
            SubCommand::with_name("connect")
                .about("Connects to a target")
                .arg(
                    Arg::with_name("hostname")
                        .short("h")
                        .help("hostname to connect to")
                        .takes_value(true)
                        .required(true)
                        .value_name("HOST"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .help("port to connect to")
                        .takes_value(true)
                        .required(true)
                        .value_name("PORT")
                        .default_value("22"),
                ),
        )
        .setting(AppSettings::SubcommandRequired);

    let matches = app.get_matches();
    match matches.subcommand() {
        ("generate", _) => generate_ip(),
        ("connect", Some(m)) => connect(
            m.value_of("hostname").unwrap(),
            m.value_of("port")
                .unwrap()
                .parse::<u16>()
                .expect("unable to parse port"),
        ),
        _ => unreachable!(),
    }
}

fn build_replaced_ip<S: Into<String>>(secret: S, ip: S) -> Result<Ipv6Addr, Box<dyn Error>> {
    let auth = otpauth::TOTP::from_base32(secret).unwrap(); // TODO: fix error handling
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let code = auth.generate(30, ts.as_secs());

    let raw_ip = build_ip(&ip.into(), code)?;
    Ok(raw_ip.parse().unwrap()) // TODO: fix error handling
}

fn generate_ip() {
    let ip_template = match env::var("TOSH_IP_TEMPLATE") {
        Ok(val) => val,
        Err(err) => panic!(
            "failed to read environment variable TOSH_IP_TEMPLATE: {}",
            err
        ),
    };

    let totp_secret = match env::var("TOSH_TOTP_SECRET") {
        Ok(val) => val,
        Err(err) => panic!(
            "failed to read environment variable TOSH_TOTP_SECRET: {}",
            err
        ),
    };

    let ip = build_replaced_ip(totp_secret, ip_template).unwrap();
    println!("{}", ip);
}

fn fmt_ipv6_expanded(addr: &Ipv6Addr) -> String {
    let mut built = String::default();
    for (idx, segment) in addr.segments().iter().enumerate() {
        let formatted = match idx {
            6 => format!(
                "{}xx",
                format!("{:04x}", segment)
                    .chars()
                    .take(2)
                    .collect::<String>()
            ), // XXX: lazy and ugly!
            7 => "xxxx".to_string(),
            _ => format!("{:04x}", segment),
        };
        built.push_str(formatted.as_str());
        if idx < 7 {
            built.push(':');
        }
    }
    built
}

fn connect(host: &str, port: u16) {
    let totp_secret = match env::var("TOSH_TOTP_SECRET") {
        Ok(val) => val,
        Err(err) => panic!(
            "failed to read environment variable TOSH_TOTP_SECRET: {}",
            err
        ),
    };

    let available_addrs: Vec<SocketAddr> = format!("{}:{}", host, port)
        .to_socket_addrs()
        .unwrap()
        .filter(|addr| addr.is_ipv6())
        .collect();

    if available_addrs.is_empty() {
        eprintln!("[tosh] could not resolve any ipv6 addresses for {}", host);
        process::exit(1);
    }

    // Pick first address, turn it into a template and then replace TOTP into it
    let orig_ip = available_addrs[0].ip();
    let ip_template = match orig_ip {
        IpAddr::V6(ip) => fmt_ipv6_expanded(&ip),
        _ => unreachable!(),
    };

    let ip = build_replaced_ip(&totp_secret, &ip_template).unwrap();
    eprintln!("[tosh] resolved {} ({}) -> {}", orig_ip, ip_template, &ip);

    // Build new socket address
    let target_addr = SocketAddr::from((ip, port));
    if let Err(err) = do_proxy(target_addr) {
        eprintln!("[tosh] failed to proxy: {}", err);
    }

    process::exit(1);
}

fn do_proxy(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut conn = TcpStream::connect(addr)?;
    let mut conn_w = conn.try_clone()?;

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let reader = thread::spawn(move || {
        io::copy(&mut conn, &mut stdout).unwrap();
    });
    let writer = thread::spawn(move || {
        io::copy(&mut stdin, &mut conn_w).unwrap();
    });

    reader.join().unwrap(); // TODO: fix error handling
    writer.join().unwrap(); // TODO: fix error handling

    Ok(())
}
