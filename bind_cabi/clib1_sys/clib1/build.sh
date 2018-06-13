#! /bin/bash

gcc -c -o Square.o Square.c
ar rcs libclib1.a Square.o
rm Square.o
