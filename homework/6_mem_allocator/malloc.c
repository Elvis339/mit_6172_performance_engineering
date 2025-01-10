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

#define PAGE_SIZE 4096
#define BLOCK_SIZE 64 // min allocation unit (must be aligned)
#define BITMAP_SIZE (PAGE_SIZE / BLOCK_SIZE)  // number of blocks in a page

// Memory pool and bitmap
static void* memory_pool = NULL;
static uint8_t bitmap[BITMAP_SIZE] = {0};

int find_free_block(size_t blocks_needed) {
  if (blocks_needed > BITMAP_SIZE) {
    return -1;
  }

  size_t consecutive = 0;
  for (size_t i = 0; i < blocks_needed; i++) {
    if (bitmap[i] == 0) {
      consecutive++;
      if (consecutive == blocks_needed) {
        return i - blocks_needed + 1;
      }
    } else {
      consecutive = 0;
    }
  }
  return -1;
}

void mark_blocks(int start, size_t blocks, int used) {
  for (size_t i = 0; i < blocks; i++) {
    bitmap[start + i] = used;
  }
}

void* packed_malloc(size_t size) {
  if (size == 0) {
    return NULL;
  }

  if (!memory_pool) {
    memory_pool = mmap(NULL, PAGE_SIZE, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (memory_pool == MAP_FAILED) {
      perror("mmap failed");
      return NULL;
    }
  }

  // Add header size and align to block size
  size_t total_size = size + sizeof(header_t);
  size_t blocks_needed = (total_size + BLOCK_SIZE - 1) / BLOCK_SIZE;

  // Find free blocks
  int start_block = find_free_block(blocks_needed);
  if (start_block == -1) {
    return NULL;  // OOM
  }

  // Mark the blocks as used
  mark_blocks(start_block, blocks_needed, 1);

  // Calculate the pointer to the allocated memory
  void* block_start = (void*)((uintptr_t)memory_pool + start_block * BLOCK_SIZE);
  header_t* header = (header_t*)block_start;
  header->size = total_size;
  header->magic = MAGIC_NUMBER;

  // Return a pointer to the user memory (after the header)
  return (void*)(header + 1);
}

void packed_free(void* ptr) {
  if (!ptr) {
    return;
  }

  // get header
  header_t* header = (header_t*)ptr - 1;

  if (header->magic != MAGIC_NUMBER) {
    fprintf(stderr, "Error: memory corruption\n");
    return;
  }

  uintptr_t block_start = (uintptr_t)header - (uintptr_t)memory_pool;
  size_t start_block = block_start / BLOCK_SIZE;
  size_t blocks_used = (header->size + BLOCK_SIZE - 1) / BLOCK_SIZE;

  mark_blocks(start_block, blocks_used, 0);
}

int main() {
  return 0;
}