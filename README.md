# tosh

Imagine your SSH server only listens on an IPv6 address, and where the last 6 digits are changing every 30 seconds as a TOTP code...

Inspired from [this tweet](https://twitter.com/kistel/status/1395375108315824130) [(Wayback machine)](https://web.archive.org/web/20210521215858/https://twitter.com/kistel/status/1395375108315824130)

Looking for a way simpler, bash implementation? Check out [old](https://github.com/mikroskeem/tosh/tree/old) branch.

## Notes

This was made because... I could make it, not if I should make it. Yes, you read it right - it's a toy.
Only use it if you know what you are doing. I am not up to handholding, preventing any footguns nor basic support requests.

Its purpose is just to add a layer of obscurity, it's probably only effective against bots (allthough most of them disappear after moving on to IPv6)
and script kiddies. If you're being targeted by e.g government agencies or people who definitely know what they do, then this probably won't help you.

Using this on top of unconfigured (in other words, running stock configuration) SSH server is always a bad idea, so please configure your SSH server
to e.g do only public key authentication, disable login for unnecessary users (e.g allow only members of group `canssh` to login) etc.

To make things more fun, you may want to adjust your firewall rules to forward to [SSH tarpit](https://github.com/skeeto/endlessh) by default.

Besides that, you NEED to ensure that your server and client times are in sync. You might want to look into [chrony](https://chrony.tuxfamily.org/).

## Usage

Assign yourself an IPv6 subnet, replace last 6 hex characters with `x`.

`fd15:4ba5:5a2b:1008:20c:29ff:fe1a:9587` -> `fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx`

Create a base32 TOTP secret, using e.g `gen-oath-safe mikroskeem totp`

```sh
$ export TOSH_IP_TEMPLATE=fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx
$ export TOSH_TOTP_SECRET=3OBVZP4AI74OIJO5YGV3UEXKXS6ISJ6H
$ tosh generate
fd15:4ba5:5a2b:1008:20c:29ff:fe59:3001
```

### Example setups

- systemd timer & iptables setup - see [examples/iptables/](examples/iptables/)

## Roadmap
- [x] Describe example setup with `iptables` & systemd
- [ ] `ssh` wrapper (`ProxyCommand` feature?)

## FAQ

### Why Rust?

I am looking forward to building a cross-platform program easily, which works even on Windows.

### Where's client?

Not done yet. Reference implementation will work inside ssh ProxyCommand option.
