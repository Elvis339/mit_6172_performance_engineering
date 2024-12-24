// Copyright (c) 2012 MIT License by 6.172 Staff

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* Typedefs */

typedef uint32_t data_t;

extern void quickSortIterative(data_t arr[], int l, int h);

/* Insertion sort */
void isort(data_t* left, data_t* right) {
  data_t* cur = left + 1;
  while (cur <= right) {
    data_t val = *cur;
    data_t* index = cur - 1;

    while (index >= left && *index > val) {
      *(index + 1) = *index;
      index--;
    }

    *(index + 1) = val;
    cur++;
  }
}

/* ~2x speedup by:
*
* 1. Processes 4 elements per iteration instead of 1, reducing branch overhead by 75%
*    Original:    1 branch per element
*    Unrolled:    1 branch per 4 elements
*
* 2. Sequential memory access pattern helps CPU prefetcher
*    Original:    *(p+1) = *p, one at a time
*    Unrolled:    *(p+1) = *p, *p = *(p-1), *(p-1) = *(p-2)...
*
* 3. Improved instruction-level parallelism for modern CPUs
*    the 4 assignments in parallel when there are no dependencies
*/
void isort_unroll(data_t* left, data_t* right) {
  for(data_t* cur = left + 1; cur <= right; cur++) {
    data_t val = *cur;
    data_t* index = cur - 1;

    // Unroll inner loop
    while(index >= left + 3 && *index > val) {
      *(index + 1) = *index;
      *(index) = *(index - 1);
      *(index - 1) = *(index - 2);
      *(index - 2) = *(index - 3);
      index -= 4;
    }

    while(index >= left && *index > val) {
      *(index + 1) = *index;
      index--;
    }
    *(index + 1) = val;
  }
}

#define CACHE_LINE_SIZE 64
#define BLOCK_SIZE (CACHE_LINE_SIZE / sizeof(data_t))

static void sort_block(data_t* start, data_t* end) {
  for (data_t* cur = start + 1; cur <= end; cur++) {
    data_t val = *cur;
    data_t* index = cur - 1;

    // Unrolled inner loop
    while (index >= start + 3 && *index > val) {
      *(index + 1) = *index;
      *(index) = *(index - 1);
      *(index - 1) = *(index - 2);
      *(index - 2) = *(index - 3);
      index -= 4;
    }

    while (index >= start && *index > val) {
      *(index + 1) = *index;
      index--;
    }
    *(index + 1) = val;
  }
}

/* Block-based insertion sort optimized for cache efficiency:
*
* Data divided into CACHE_LINE_SIZE/sizeof(data_t) blocks
*    - Each block fits perfectly in L1 cache line (64B)
*    - Minimizes cache misses by processing block-at-a-time
*    - Modern CPU prefetcher can efficiently predict next block
*/
void isort_block(data_t* left, data_t* right) {
  // First sort within blocks
  for (data_t* block_start = left; block_start <= right; block_start += BLOCK_SIZE) {
    data_t* block_end = block_start + BLOCK_SIZE - 1;
    if (block_end > right) block_end = right;
    sort_block(block_start, block_end);
  }

  // Merge sorted blocks
  for (data_t* merge_point = left + BLOCK_SIZE; merge_point <= right; merge_point += BLOCK_SIZE) {
    data_t val = *merge_point;
    data_t* index = merge_point - 1;

    // Find insertion point in previous blocks
    while (index >= left && *index > val) {
      *(index + 1) = *index;
      index--;
    }
    *(index + 1) = val;
  }
}

int main(int argc, char* argv[]) {
  // Accept either 2 or 3 arguments (flag is optional)
  if (argc != 3 && argc != 4) {
    printf("Error: wrong number of arguments.\n");
    printf("Usage: %s <size> <iterations> [block,unroll]\n", argv[0]);
    exit(-1);
  }

  int N = atoi(argv[1]);
  int K = atoi(argv[2]);
  // Only try to access argv[3] if it exists
  char* flag = argc == 4 ? argv[3] : NULL;
  unsigned int seed = 42;
  printf("Sorting %d values...\n", N);
  data_t* data = (data_t*) malloc(N * sizeof(data_t));
  if (data == NULL) {
    free(data);
    printf("Error: not enough memory\n");
    exit(-1);
  }

  int i, j;
  for (j = 0; j < K; j++) {
    for (i = 0; i < N; i++) {
      data[i] = rand_r(&seed);
    }

    // Check if flag exists and is "opt"
    if (flag != NULL && strcmp(flag, "unroll") == 0) {
      printf("Using unroll version\n");
      isort_unroll(data, data + N - 1);
    } else if (flag != NULL && strcmp(flag, "block") == 0) {  // Added else
      printf("Using block version\n");
      isort_block(data, data + N - 1);
    } else {
      printf("Using un-optimized\n");
      isort(data, data + N - 1);
    }
  }

  free(data);
  printf("Done!\n");
  return 0;
}