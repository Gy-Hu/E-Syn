for eqn in $(find . -name "*.eqn"); do
  eqn_path=$(dirname "$eqn")
  base_name=$(basename "$file" .eqn)
  abc -c "read_eqn $eqn ; strash; write_eqn $eqn"
done

