#!/bin/bash

# may need short_names/strash
# short_names: after read bench (sometimes) , before write eqn
# strash: after read bench (sometimes) , before write aiger

for file in $(find . -name "*.bench"); do
  base_name=$(basename "$file" .bench)
  abc -c "read_bench $file ; short_names; write_eqn ${base_name}.eqn"
done