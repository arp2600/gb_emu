#!/bin/sh

bench=$(cargo run --example benchmark 2> /dev/null | sed -n -e 's/^_BENCH_ //p')

date >> benchmark_results
echo "$bench" >> benchmark_results
echo "" >> benchmark_results
