#!/bin/bash

usage() {
    echo "Usage: $0 -d <day> [-i <input>]" 1>&2
    exit 1
}

while getopts ":d:i:" opt; do
  case $opt in
    d)
        day=$(printf "%02d" $OPTARG)
        ;;
    i)
        input=$OPTARG
        ;;
    *)
        usage
        ;;
  esac
done

if [ -z "$day" ]; then
    usage
fi
if [ -z "$input" ]; then
    input="input/day$day"
fi

cargo run --release -p day$day < $input
