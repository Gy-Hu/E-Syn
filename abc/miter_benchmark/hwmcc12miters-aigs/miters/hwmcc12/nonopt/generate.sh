#!/bin/sh
cd `dirname $0`
rm -f *.aig 
for i in ../moved/*.aig
do
  b=`basename $i .aig`
  echo -n "$b [`head -1 $i`]"
  rm -f $b.aig
  aigmiter -c $i $i > $b.aig
  echo " non optimized miter [`head -1 $b.aig`]"
done
