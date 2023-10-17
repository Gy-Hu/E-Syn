#!/bin/sh
cd `dirname $0`
rm -f *.aig
for i in ../original/*.aig
do
  b=`basename $i .aig`
  echo -n "$b [`head -1 $i`]"
  n=`head -1 $i|awk '{print $5}'`
  if [ $n -gt 0 ]
  then
    aigreset $i $b.aig
    echo -n " only reset to "
  else
    aigmove $i |aigreset > $b.aig
    echo -n " moved and reset to "
  fi
  echo "[`head -1 $b.aig`]"
done
