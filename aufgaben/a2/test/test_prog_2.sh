#!/bin/sh

echo 10 | ../build/njvm --prog2 > out/prog2.out
echo 10 | ./njvm --prog2 > out/target.prog2.out

if diff -bc out/prog2.out out/target.prog2.out;
then
    echo "PASSED" 
else
    echo "FAILED" 
    exit 1
fi
