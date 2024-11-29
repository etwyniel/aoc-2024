#!/bin/sh

day=$(printf '%02d' ${1-1})

day_file=src/day$day.rs

[ -f $day_file ] && exit 0

cp template.rs "$day_file"
sd W $day "$day_file"

sd 'fn main' "mod day$day;\nuse day$day::Day$day;\n\nfn main" src/main.rs
sd ']' "Day$day::run,\n]" src/main.rs