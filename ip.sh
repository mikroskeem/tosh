#!/usr/bin/env bash
set -euo pipefail

LANG=C

# gen-oath-safe mark totp
#SECRET=db835cbf8047f8e425ddc1abba12eabcbc8927c7
SECRET="3OBVZP4AI74OIJO5YGV3UEXKXS6ISJ6H"

IP_TEMPLATE="fd15:4ba5:5a2b:1008:20c:29ff:fexx:xxxx"
PORT=22


code=$(./totp <<< "${SECRET}")

ADDR=""
j=0
for (( i=0; i<${#IP_TEMPLATE}; i++ )); do
	chr="${IP_TEMPLATE:$i:1}"
	if [ "${chr}" = "x" ]; then
		num=${code:$j:1}	
		ADDR="${ADDR}${num}"
		(( j = j+1 ))

		if [ "${j}" -gt 6 ]; then
			echo ">>> Template contains too many 'x'-es (max 6, got $j)"
		fi
	else
		ADDR="${ADDR}${chr}"
	fi
done

echo ">>> ${code} -> ${ADDR}"
printf '%s\n' --destination "${ADDR}" --dport "${PORT}"
