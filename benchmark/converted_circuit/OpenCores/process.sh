#!/bin/bash

# may need short_names/strash
# short_names: after read bench (sometimes) , before write eqn
# strash: after read bench (sometimes) , before write aiger

for file in $(find . -name "*.aig"); do
  base_name=$(basename "$file" .aig)
  abc -c "read_aiger $file ; write_eqn ${base_name}.eqn"
done