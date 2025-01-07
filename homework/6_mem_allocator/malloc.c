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

  // adjust size to ensure 8-byte alignment by rounding up to the nearest multiple of 8
  // This bit manipulation technique works by:
  // 1. Adding 7 to size to prepare for rounding
  // 2. Using bitwise AND with inverted 7 to clear the last 3 bits
  // Example: size = 21
  // 21 + 7 = 28    (0001 1100)
  // ~7     =       (1111 1000)
  // Result  =      (0001 1000) = 24
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

// must be 8-byte aligned itself
typedef struct header {
  size_t size;        // total size including header
  uint32_t magic;     // magic number to detect corruption
} header_t;

// used to detect memory corruption and helps identify
// if someone wrote before the start of their allocated block
#define MAGIC_NUMBER 0xDEADBEEF

/**
* Low Address  [Header][....User Data....]  High Address
            ↑       ↑
            |       |
            |       +-- Pointer returned to user
            +-- Actual start of allocated block
*/
void* wrapped_malloc(size_t size) {
  if (size == 0) {
    return NULL;
  }

  size_t total_size = size + sizeof(header_t);
  size_t aligned_size = (total_size + 7) & ~7;

  void* block = sbrk(aligned_size);
  if (block == (void*)-1) {
    return NULL;  // OOM
  }

  // init header
  header_t* header = (header_t*)block;
  header->size = aligned_size;
  header->magic = MAGIC_NUMBER;

  // pointer to memory (just past header)
  return (void*)(header + 1);
}