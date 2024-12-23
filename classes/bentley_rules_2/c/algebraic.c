#include "math.h"
#include <stdbool.h>
#include <stdio.h>

typedef struct {
  double x;
  double y;
  double z;
  double r;
} ball_t;

double square(double x) { return x * x; }

bool collides(ball_t *a, ball_t *b) {
  double d =
      sqrt(square(a->x - b->x) + square(a->y - b->y) + square(a->z - b->z));

  return d <= a->r + b->r;
}

// use algebraic identities to speed up computation
// sqrt is expensive
// sqrt(u) <= v exactly when u <= y^2
bool collides_opt(ball_t *a, ball_t *b) {
  double dsquared =
      square(a->x - b->x) + square(a->y - b->y) + square(a->z - b->z);

  return dsquared <= square(a->r + b->r);
}

int main(int argc, const char *argv[]) {
  ball_t a = {.x = 1, .y = 1, .z = 1, .r = 0};
  ball_t b = {.x = 0, .y = 0, .z = 0, .r = 0};

  bool c = collides(&a, &b);
  printf("%s\n", c ? "true" : "false");

  return 0;
}