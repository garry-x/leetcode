#!/bin/bash

if [ $# -lt 1 ]; then
    echo "$0 \"solution_name\""
    exit 1
fi

prob_name=$1

# clean project

cd $prob_name

cargo clean

cd ..

git add *

git commit -s

git push
