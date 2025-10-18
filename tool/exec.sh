#!/usr/bin/env bash
set -e
fileBasenameNoExtension=$1
cargoOptions=$2

INPUT_PATH=test/in
OUTPUT_PATH=test/out #/dev/stdout
ulimit -s 32768
export RUST_BACKTRACE=1

if [ "$(uname)" == "Darwin" ]; then # Mac OS X platform
	time cargo run --bin ${fileBasenameNoExtension} --release $cargoOptions <$INPUT_PATH >$OUTPUT_PATH 2> >(awk '/User/||/System/||/Elap/||/Max/' > /dev/stderr)
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then # GNU/Linux platform
	/usr/bin/time -v cargo run --bin ${fileBasenameNoExtension} --release $cargoOptions <$INPUT_PATH >$OUTPUT_PATH 2> >(awk '/User/||/System/||/Elap/||/Max/' > /dev/stderr)
fi
