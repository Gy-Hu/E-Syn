#!/bin/sh
cd `dirname $0`
rm -f *.aig
version=`abc -c version|awk '{print $4}'`
for i in ../moved/*.aig
do
  b=`basename $i .aig`
  echo -n "$b [`head -1 $i`]"
  #script="dc2; dch; balance -x"
  script="dc2;zero"
  abc -c "read $i; $script; write $b.aig"
  echo " optimized by ABC $version ('$script') to [`head -1 $b.aig`]"
done
