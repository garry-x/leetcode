#!/bin/bash

if [ $# -lt 2 ]; then
    echo "$0 \"solution_name\" \"id\""
    exit 1
fi

prob_name=$1
prob_id=$2

# create project
cargo new --bin --vcs none $prob_name

mv $prob_name "${prob_id}-${prob_name}"

# copy template
cp template/src/main.rs ${prob_id}-${prob_name}/src/ 
