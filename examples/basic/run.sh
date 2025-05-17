#!/bin/sh

zi0="./sample.d/z0.zip"
zi1="./sample.d/z1.zip"

ENV_JSON_GZIPPED=true
ENV_JSON_GZIPPED=false

geninput(){
	echo creating input zip files...

	mkdir -p sample.d

	jq -c -n '{name: "fuji",  height: 3.776}' > sample.d/z0j0.json
	jq -c -n '{name: "takao", height: 0.599}' > sample.d/z0j1.json

	jq -c -n '{name: "FUJI",  height: 3.776}' > sample.d/z1j0.json
	jq -c -n '{name: "TAKAO", height: 0.599}' > sample.d/z1j1.json

	find sample.d -type f -name 'z0*.json' |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			"${zi0}"

	find sample.d -type f -name 'z1*.json' |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			"${zi1}"

}

test -f "${zi0}" || geninput
test -f "${zi1}" || geninput

ls "${zi0}" "${zi1}" |
	sed \
		-n \
		-e s/sample.d/guest.d/ \
		-e s/.// \
		-e p |
	wazero \
		run \
		-env ENV_JSON_GZIPPED=${ENV_JSON_GZIPPED} \
		-mount "./sample.d:/guest.d:ro" \
		./basic.wasm
