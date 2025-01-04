//
// Created by Elvis Sabanovic on 04/01/2025.
//

#include "malloc.h"

/**
* Before allocation:
+------------------+--------------------------------+
| Used Memory      | Free Memory                    |
+------------------+--------------------------------+
                   ↑
                   break

After simple_malloc(24):
+------------------+----------------+----------------+
| Used Memory      | New Block (24) | Free Memory   |
+------------------+----------------+----------------+
                                   ↑
                                   new break

- it never reuses memory it keeps allocating (keeps growing never shrinking)
- it doesn't keep track of block sizes
*/
void* simple_malloc(size_t size) {
  if (size == 0) {
    return NULL;
  }

  // adjust size for alignment
  // this is known bit trick for  rounding down to the nearest multiple of 8
  //  7 = 0 0 0 0 0 1 1 1
  // ~7 = 1 1 1 1 1 0 0 0
  // Example: size = 21
  // 21 = 0 0 0 1 0 1 0 1
  // ~7 = 1 1 1 1 1 0 0 0
  // 21 & ~7 = 0 0 0 1 0 0 0 0 = 16
  size_t aligned_size = (size + 7) & ~7;

  // current break point
  void* current = sbrk(0);

  // shift break point up by aligned_size
  if (sbrk(aligned_size) == (void*)-1) {
    return NULL;    // Error case
  }

  // return the old break point
  return current;
}
