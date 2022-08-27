#!/bin/bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cat just_a_test.txt > "$OUTDIR/test_1.txt"
cat just_a_test.txt | tr a-z A-Z > "$OUTDIR/test_2.txt"

