//
// Created by Elvis Sabanovic on 23/12/2024.
//
#include <stdio.h>

int64_t fibonacci(int64_t n) {
  if (n < 2)
    return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}

int main() {
  int64_t n = 12;
  printf("%lld\n", fibonacci(n));
  return 0;
}