#!/bin/bash

day="$1"
if [[ -z $day ]]; then
    echo "need day"
    exit 1
fi

path="src/day${day}.rs"
touch $path

test_path="input/${day}_test.input"
touch $test_path

cat << EOF > $path
use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("${day}_test.input");
    0
}

fn part_2() -> u64 {
    0
}

run!();
EOF

out="input/${day}.input"
if [ ! -f $out ]; then
    session=$(cat .env)
    CMD="curl --silent --ssl-no-revoke -b session=${session} https://adventofcode.com/2023/day/${day}/input -o ${out}"
    ${CMD}
fi

cat << EOF > "src/main.rs"
#![allow(dead_code)]

#[rustfmt::skip]
mod util;
mod day${day};

fn main() {
    day${day}::run();
}
EOF

subl $out
subl $test_path
subl $path

DEBUG=1 cargo run
