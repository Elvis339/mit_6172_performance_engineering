//
// Created by Elvis Sabanovic on 19/12/2024.
//
#include <stdlib.h>
#include <stdio.h>
#include "sys/time.h"
#include <cilk/cilk.h>

#define N 4096

double A[N][N];
double B[N][N];
double C[N][N];

float time_diff(struct timeval *start, struct timeval *end) {
  return (end->tv_sec - start->tv_sec) + 1e-6 * (end->tv_usec - start->tv_usec);
}

int main(int argc, const char *argv[]) {
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      A[i][j] = (double)rand() / (double)RAND_MAX;
      B[i][j] = (double)rand() / (double)RAND_MAX;
      C[i][j] = 0.0;
    }
  }

  struct timeval start, end;
  gettimeofday(&start, NULL);

  // two level tiling mat mul
  int s = 64;
  int t = 64;

  cilk_for (int ih = 0; ih < N; ih += s) {
    cilk_for (int jh = 0; jh < N; jh += s) {
      for (int kh = 0; kh < N; kh += s) {
        for (int im = 0; im < s; im += t) {
          for (int jm = 0; jm < s; jm += t) {
            for (int km = 0; km < s; km += t) {
              for (int il = 0; il < t; ++il) {
                for (int kl = 0; kl < t; ++kl) {
                  for (int jl = 0; jl < t; ++jl) {
                    C[ih+im+il][jh+jm+jl] +=
                      A[ih+im+il][kh+km+kl] * B[kh+km+kl][jh+jm+jl];
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  gettimeofday(&end, NULL);
  printf("%0.6f\n", time_diff(&start, &end));
  return 0;
}