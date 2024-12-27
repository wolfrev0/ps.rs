#!/usr/bin/env bash
ulimit -s 32768

RED='\033[1;31m'
CYAN='\033[1;36m'
GREEN='\033[1;32m'
NONE='\033[0m'
shopt -s nullglob
# set -e
# trap "echo -e '${RED}FAIL${NONE}'" ERR
test_cnt=$(( $(find test/ -name "*.in" -printf '.' | wc -m)+1 ))
succ=0
fail=0

input_pattern=test/*.in
for i in $input_pattern;
do
	answer_pattern=${i%.*}.ans
	echo "Running "${i%.*}" ("$(( ($succ+$fail)*100/$test_cnt ))"%)"
	temp_output=$(mktemp)
	temp_stderr=$(mktemp)
	tool/exec.sh $1 -q < $i > $temp_output 2> $temp_stderr
	echo $(cat $temp_stderr)
	echo $(cat $temp_output)
	echo $(cat $answer_pattern)
	if diff $temp_output $answer_pattern -u -B; then
		succ=$((succ+1))
	else
		fail=$(($fail+1))
		fail_list+=$i", ";
	fi
done
# input_pattern=test/in
# answer_pattern=test/ans

echo -e "${GREEN}Ok: $succ tests${NONE}"
if (( $fail )); then
	echo -e "${RED}Fail: $fail tests. [$fail_list] ${NONE}"
	exit 1;
fi