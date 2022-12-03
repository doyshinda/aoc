#!/bin/bash

day="$1"
if [[ -z $day ]]; then
    echo "need day"
    exit 1
fi

path="src/day${day}.rs"
touch $path

cat << EOF > $path
use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("${day}.input");
    log!("part 1 data len: {}", data.len());
    0
}

fn part_2() -> u64 {
    log!("Ran part 2");
    0
}

run!();
EOF

out="input/${day}.input"
if [ ! -f $out ]; then
    session=$(cat .env)
    CMD="curl --silent --ssl-no-revoke -b session=${session} https://adventofcode.com/2020/day/${day}/input -o ${out}"
    ${CMD}
fi

DEBUG=1 cargo run
