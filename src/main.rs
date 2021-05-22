use std::env;
use std::net::Ipv6Addr;
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
            m.value_of("port").unwrap().parse::<u16>().unwrap(),
        ),
        _ => unreachable!(),
    }
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

    let auth = otpauth::TOTP::from_base32(totp_secret).unwrap();
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let code = auth.generate(30, ts.as_secs());

    let raw_ip = build_ip(&ip_template, code).unwrap();
    let ip: Ipv6Addr = raw_ip.parse().unwrap();

    println!("{}", ip);
}

fn connect(host: &str, port: u16) {
    println!("connecting to {}:{}", host, port);
}
