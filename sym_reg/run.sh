#!/bin/bash
#BSUB -J pono-coi-nosort[1-324]
#BSUB -n 1
#BSUB -W 01:10
#BSUB -e %J-%I.err
#BSUB -o %J-%I.out


i=$LSB_JOBINDEX
timelimit=3600
dirname="/hpc/home/hongcezh/hwmcc/hwmcc20"
fname=$(head -$i /hpc/home/hongcezh/hwmcc/hwmcc20/flist20-bv-btor.txt | tail -n 1)
echo "checking:" ${dirname}/${fname}
/usr/bin/time timeout $timelimit /hpc/home/hongcezh/pono/build/pono-bwd-sort -e ic3bits --ic3-no-sort-lemma -k 1000 ${dirname}/${fname}
echo "----END----"