//
// Created by Elvis Sabanovic on 04/01/2025.
//

#include <stdio.h>
#include <unistd.h>
#include <stddef.h>
#include <sys/mman.h>
#include <stdint.h>
#include <string.h>


#ifndef MALLOC_H
#define MALLOC_H

void *simple_malloc(size_t size);

void *wrapped_malloc(size_t size);

// packed_malloc optimizes for space efficiency by packing allocations tightly together
void *packed_malloc(size_t size);
void packed_free(void* ptr);

#endif //MALLOC_H
