#include <math.h>

void scale(double *X, double *Y, int N) {
  for (int i = 0; i < N; i++) {
    Y[i] = X[i] * exp(sqrt(M_PI / 2));
  }
}

void scale_optimized(double *X, double *Y, int N) {
  // no need to recompute factor each iteration
  double factor = exp(sqrt(M_PI / 2));
  for (int i = 0; i < N; i++) {
    Y[i] = X[i] * factor;
  }
}

int main() { return 0; }