#!/bin/bash

rm -rf main.build
clang main.c -o main.build

if [ $? -eq 0 ]; then
    echo "Hello, compiled successful!"
else
    echo "Compilation failed!"
fi