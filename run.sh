#!/bin/bash

day=$(printf "%02d" $1)
cargo run --release -p day$day < input/day$day
