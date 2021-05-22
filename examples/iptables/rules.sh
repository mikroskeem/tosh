#!/usr/bin/env bash
set -euo pipefail

ip6tables -t filter -N TOSH
ip6tables -t filter -A TOSH -m set --match-set tosh-ips src -j ACCEPT
ip6tables -t filter -A TOSH -p tcp -j REJECT --reject-with tcp-reset

ip6tables -t filter -A INPUT -p tcp -m tcp --dport 22 -i ens33 -j TOSH
