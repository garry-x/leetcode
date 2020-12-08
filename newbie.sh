#!/bin/bash

if [ $# -lt 1 ]; then
    echo "$0 \"solution_name\""
    exit 1
fi

prob_name=$1

# create project
cargo new --bin --vcs none $prob_name

# copy template
cp template/src/main.rs ${prob_name}/src/ 
