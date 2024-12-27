#!/usr/bin/env bash

fileBasenameNoExtension=$1
rm -f submit.rs && cargo equip --bin ${fileBasenameNoExtension} --remove docs --minify libs >> submit.rs