//
// Created by Elvis Sabanovic on 04/01/2025.
//

#include <stdio.h>
#include <unistd.h>
#include <stddef.h>

#ifndef MALLOC_H
#define MALLOC_H

void *simple_malloc(size_t size);

void *wrapped_malloc(size_t size);

#endif //MALLOC_H
