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
    let mut ans = 0;

    ans
}

fn part_2() -> u64 {
    let data = util::read_input("${day}_test.input");
    let mut ans = 0;

    ans
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}

EOF

out="input/${day}.input"
if [ ! -f $out ]; then
    session=$(cat .env)
    CMD="curl --silent --ssl-no-revoke -b session=${session} https://adventofcode.com/2024/day/${day}/input -o ${out}"
    ${CMD}
    CHAR=$(tail -c -1 $out)
    hex="$(echo "$CHAR" | hexdump -ve '/1 "%02X"')"
    if [[ "$hex" == "0A" ]]; then
        head -c -1 $out > /tmp/foo.txt
        mv /tmp/foo.txt $out
        rm -rf /tmp/foo.txt
    fi
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
google-chrome "https://adventofcode.com/2024/day/${day}"
