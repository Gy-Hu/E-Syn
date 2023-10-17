#!/bin/bash

echo "name, seq, simple_CO, line_count, file_size, score"

results=()

for log in $(find /data/guangyuh/coding_env/E-Brush/benchmark/converted_circuit -name "log.txt"); do
  seq=0
  simple_CO=0
  log_dir=$(dirname "$log")
  while read -r line; do
    if [[ $line == *"Warning: only combinational portion is being written."* ]]; then
      seq=1
    elif [[ $line == *"The network is converted to have simple COs"* ]]; then
      simple_CO=1
    elif [[ $line == *"ABC command line"* ]]; then
      name=$(echo "$line" | awk -F'./' '{print $2}' | awk -F' ' '{print $1}')
      full_name="${log_dir}/${name}"
      full_name_eqn="${full_name/.bench/.eqn}"
      full_name_eqn="${full_name_eqn/.blif/.eqn}"
      full_name_eqn="${full_name_eqn/.aig/.eqn}"

      # if [ -e "$full_name_eqn" ]; then
      if [ -e "$full_name_eqn" ] && [ $seq -eq 0 ] ; then
      #if [ -e "$full_name_eqn" ] && [ $seq -eq 0 ] && [ $simple_CO -eq 0 ]; then #only keep the combinational circuits
        line_count=$(wc -l < "$full_name_eqn")
        file_size=$(stat -c%s "$full_name_eqn")
        score=$(echo "0.7 * $file_size + 0.3 * $line_count" | bc)
        results+=("$full_name_eqn, $seq, $simple_CO, $line_count, $file_size, $score")
      fi

      seq=0
      simple_CO=0
    fi
  done < "$log"
done

IFS=$'\n'
sorted_results=($(echo "${results[*]}" | sort -t, -k6,6n))
unset IFS

for result in "${sorted_results[@]}"; do
  echo "$result"
done