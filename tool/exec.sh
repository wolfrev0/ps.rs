#!/usr/bin/env bash
set -e
fileBasenameNoExtension=$1
cargoOptions=$2

ulimit -s 32768
if [ "$(uname)" == "Darwin" ]; then # Mac OS X platform
	time cargo run --bin ${fileBasenameNoExtension} --release $cargoOptions < test/in
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then # GNU/Linux platform
	/usr/bin/time -v cargo run --bin ${fileBasenameNoExtension} --release $cargoOptions <test/in 2> >(awk '/User/||/System/||/Elap/||/Max/')
fi
