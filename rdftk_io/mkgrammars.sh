#!/usr/bin/env bash

LANGUAGES=("nq" "nt" "turtle")

for grammar in ${LANGUAGES[@]}; do
    echo cat "src/${grammar}/${grammar}-in.pest" "src/common/common.pest" ">" "src/${grammar}/${grammar}.pest"
    cat "src/${grammar}/${grammar}-in.pest" "src/common/common.pest" > "src/${grammar}/${grammar}.pest"
done
