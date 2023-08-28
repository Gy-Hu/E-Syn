#!/bin/sh
cd `dirname $0`
rm -f *.aig
for i in ../abc/*.aig
do
  b=`basename $i .aig`
  echo -n "$b [`head -1 ../moved/$b.aig` | `head -1 ../abc/$b.aig`]"
  aigmiter -c ../moved/$b.aig ../abc/$b.aig > $b.aig
  echo " optimized miter [`head -1 $b.aig`]"
done
