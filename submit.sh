#!/bin/bash

if [ $# -lt 2 ]; then
    echo "$0 \"solution_name\" \"id\"" 
    exit 1
fi

prob_name=$1
prob_id=$2

dir_name=${prob_id}-${prob_name}

if [ ! -d $dir_name ]; then
   exit 1 
fi

cd $dir_name

cargo clean

cd ..

git add *

git commit --author="Garry Xu <garry.x@outlook.com>" \
        -m "
leetcode[$prob_id]: add solution for ${prob_name}

REF: https://leetcode.com/problems/${prob_name}

Signed-off-by: Garry Xu <garry.x@outlook.com>
"

git push
