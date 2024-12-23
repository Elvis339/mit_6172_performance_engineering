//
// Created by Elvis Sabanovic on 23/12/2024.
//
#include <stdio.h>

// merge arrays
// __restrict keyword helps the compiler that the pointer points to one piece of data
// helping it make optimizations
static void merge(long* __restrict C,
                 long* __restrict A,
                 long* __restrict B,
                 size_t na,
                 size_t nb)
{
  // 4) Predictable branch because we know whether na is greater then nb beforehand
  // also, most of the time it's going to return true and in that case you're entering the execution
  while (na > 0 && nb > 0) {
    // 3) Unpredictable branch because we don't know the values of A and B beforehand
    // The hardware can't do prefetching efficiently
    if (*A <= *B) {
      *C++ = *A++;
      na--;
    } else {
      *C++ = *B++;
      nb--;
    }
  }

  // 2) Predictable for the same reason as 1.
  while (na > 0) {
    *C++ = *A++;
    na--;
  }

  // 1) This branch is predictable it's going to return true most of the time
  // except for the last time it's only going to return false
  // when `nb == 0` and at that point you're going to execute this once and then you're done
  while (nb > 0) {
    *C++ = *B++;
    nb--;
  }
}

// branchless merge arrays using bit trics (min)
static void merge_branchless(long* __restrict C,
                 long* __restrict A,
                 long* __restrict B,
                 size_t na,
                 size_t nb)
{
  while (na > 0 && nb > 0) {
    // Min bit-trick
    // this optmization works well on some machines but on modern machines
    // using clang -03 it will be slower than branching version.
    // modern compilers can perform this optimization better.
    long cmp = (*A <= *B);
    long min = *B ^ ((*B ^ *A) & (-cmp));
    *C++ += min;
    A += cmp; na -= cmp;
    B += !cmp; nb -= !cmp;
  }

  while (na > 0) {
    *C++ = *A++;
    na--;
  }

  while (nb > 0) {
    *C++ = *B++;
    nb--;
  }
}

int main() {
  /*
  In C, arrays and pointers have a very close relationship due to array decay.
  When you pass an array to a function, it automatically "decays" into a pointer to its first element.
  ```
    long a[] = {3, 12, 19, 46};   // This is an array
    long* ptr = a;                 // This is valid - 'a' decays to &a[0]
  ```
   */
  long a[] = {3, 12, 19, 46};
  long b[] = {4, 14, 21, 23};
  long c[8] = {0}; // size_a + size_b

  merge(c, a, b, 4, 4);

  for (int i = 0; i < 8; i++) {
    printf("%ld\n", c[i]);
  }

  return 0;
}