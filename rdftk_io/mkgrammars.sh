#!/usr/bin/env bash

for grammar in "nq" "nt" "turtle"
do
  echo cat "src/$grammar/$grammar-in.pest" "src/common/common.pest" ">" "src/$grammar/$grammar.pest"
  cat "src/$grammar/$grammar-in.pest" "src/common/common.pest" > "src/$grammar/$grammar.pest"
done
