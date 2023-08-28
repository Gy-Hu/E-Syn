#!/bin/sh
rm -f generate.log
first=yes
echo "START `date`" | tee -a generate.log
for i in moved nonopt abc opt
do
  if [ $first = yes ]
  then
    first=no
  else
    echo "TIMESTAMP `date`" | tee -a generate.log
  fi
  ./$i/*.sh | tee -a generate.log
done
echo "END `date`" | tee -a generate.log
