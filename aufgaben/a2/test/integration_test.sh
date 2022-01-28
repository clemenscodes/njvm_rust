#!/bin/sh

rm -rf out
mkdir out

echo "Testing --prog1 ..."
echo
./test_prog_1.sh
echo
echo "Testing --prog2 ..."
echo
./test_prog_2.sh
echo
echo "Testing --prog3 ..."
echo
./test_prog_3.sh
