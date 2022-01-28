#!/bin/sh

echo 1 | ../build/njvm --prog3 > out/prog3.out
echo 1 | ./njvm --prog3 > out/target.prog3.out

if diff -bc out/prog3.out out/target.prog3.out;
then
    echo "PASSED" 
else
    echo "FAILED" 
    exit 1
fi
