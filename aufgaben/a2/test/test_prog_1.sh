#!/bin/sh

../build/njvm --prog1 > out/prog1.out
./njvm --prog1 > out/target.prog1.out

if diff -bc out/prog1.out out/target.prog1.out;
then
    echo "PASSED" 
else
    echo "FAILED" 
    exit 1
fi
