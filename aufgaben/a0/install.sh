#!/bin/sh

gcc -g -Wall -std=c99 -pedantic -o njvm ./njvm.c && ./njvm --help