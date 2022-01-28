#!/bin/sh

rm -rf build
mkdir build && cd build || exit
cmake -DCMAKE_C_COMPILER=gcc -DCMAKE_C_FLAGS="-g -Wall -std=c99 -pedantic" ../src
make
./njvm
