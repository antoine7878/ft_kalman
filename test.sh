#!/usr/bin/env bash

failures=0

for _ in {1..20}; do
    make loop-test
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        failures=$((failures + 1))
    fi
done

echo "Command failed $failures time(s)."
