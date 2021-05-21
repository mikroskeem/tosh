use std::net::Ipv6Addr;
use std::time::{SystemTime, UNIX_EPOCH};

use google_authenticator::GoogleAuthenticator;

const IP_TEMPLATE: &str = "fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx";
const TOTP_SECRET: &str = "3OBVZP4AI74OIJO5YGV3UEXKXS6ISJ6H";

fn build_ip(base: &str, code: u32) -> Result<String, &'static str> {
    let code_str = code.to_string();
    if !code_str.is_ascii() {
        return Err("unexpected non-ascii characters while converting code to string");
    }

    let code_chars = code_str.as_bytes();
    println!("code chars: {:?} ({})", code_chars, code_str);
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
    let auth = GoogleAuthenticator::new();
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let code = auth
        .get_code(TOTP_SECRET, ts.as_secs())
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let raw_ip = build_ip(IP_TEMPLATE, code).unwrap();
    let ip: Ipv6Addr = raw_ip.parse().unwrap();
    println!(">>> {} -> {}", code, ip);
}
