#!/bin/bash

rm -rf main.build

/opt/opencilk/bin/clang main.c -o main.build -O3 -fopencilk -isysroot $(xcrun --show-sdk-path)

if [ $? -eq 0 ]; then
    echo "Matrix mul compiled successful!"
else
    echo "Compilation failed!"
fi