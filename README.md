# tosh

Imagine your SSH server only listens on an IPv6 address, and where the last 6 digits are changing every 30 seconds as a TOTP code...

Inspired from [this tweet](https://twitter.com/kistel/status/1395375108315824130) [(Wayback machine)](https://web.archive.org/web/20210521215858/https://twitter.com/kistel/status/1395375108315824130)

## Usage

Assign yourself an IPv6 subnet, replace last 6 hex characters with `x`.

`fd15:4ba5:5a2b:1008:20c:29ff:fe1a:9587` -> `fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx`

Create a base32 TOTP secret, using e.g `gen-oath-safe mark totp`

```sh
TOSH_IP_TEMPLATE=fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx
TOSH_TOTP_SECRET=3OBVZP4AI74OIJO5YGV3UEXKXS6ISJ6H

tosh
```

## Roadmap
- [ ] Describe example setup with `iptables`
- [ ] `ssh` wrapper (`ProxyCommand` feature?)
