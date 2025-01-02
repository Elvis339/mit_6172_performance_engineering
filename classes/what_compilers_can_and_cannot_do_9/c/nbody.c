//
// Created by Elvis Sabanovic on 30/12/2024.
//
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// LAW OF GRAVITATION:
// F_21 = (G m1m2/|r_12|^2) unit(r_21)

typedef struct vect_t {
  double x, y;
} vec_t;

static vec_t add(vec_t a, vec_t b) {
  vec_t sum = { a.x + b.x, a.y + b.y };
  return sum;
}

static vec_t scale(vec_t v, double a) {
  vec_t scaled = { v.x * a, v.y * a };
  return scaled;
}

static double vec_length2(vec_t v) {
  return v.x * v.x + v.y * v.y;
}

typedef struct body_t {
  vec_t position;
  vec_t velocity;
  vec_t force;
  double mass;
} body_t;

void update_position(int nbodies, body_t *bodies, double time_quantum) {
  for (int i = 0; i < nbodies; i++) {
    // compute new veolicty for each body
    vec_t new_velocity = scale(bodies[i].force, time_quantum / bodies[i].mass);

    // update position of each body based on average based on it's old and new velocity
    bodies[i].position = add(bodies[i].position, scale(add(bodies[i].velocity, new_velocity), time_quantum / 2.0));

    // set new velocity of bodies
    bodies[i].velocity = new_velocity;
  }
}

//void calculate_forces(int nbodies, body_t *bodies) {
//  for (int i = 0; i < nbodies; i++) {
//    for (int j = 0; j < nbodies; j++) {
//      if (i == j) continue;
//
//      add_force(&bodies[i], calculate_force(&bodies[i], &bodies[j]));
//    }
//  }
//}

//void calculate_forces(int nbodies, body_t *bodies) {
//  for (int i = 0; i < nbodies; i++) {
//    This change computes address on each j loop
//    we can hoist it to compute only when i ticks and re-use it in j loop
//    body_t *bi = &bodies[i];
//    for (int j = 0; j < nbodies; j++) {
//      if (i == j) continue;
//
//      add_force(body_t, calculate_force(body_t, &bodies[j]));
//    }
//  }
//}

void simulate(body_t *bodies, int nbodies, int nsteps, double time_quantum) {
  for (int i = 0; i < nbodies; i++) {
//    calculate_forces(nbodies, bodies);
    update_position(nbodies, bodies, time_quantum);
  }
}

int main() {
  srand(time(NULL));
  const int nbodies = 3;
  const int nsteps = 100;
  const double time_quantum = 0.1;

  body_t bodies[nbodies];

  // Initialize random bodies
  for (int i = 0; i < nbodies; i++) {
    bodies[i] = (body_t){
      .position = {(double)rand()/RAND_MAX * 20 - 10, (double)rand()/RAND_MAX * 20 - 10},
      .velocity = {0, 0},
      .force = {0, 0},
      .mass = (double)rand()/RAND_MAX * 100
  };
  }

  simulate(bodies, nbodies, nsteps, time_quantum);

  for (int i = 0; i < nbodies; i++) {
    printf("Body %d: Position (%.2f, %.2f)\n", i, bodies[i].position.x, bodies[i].position.y);
  }

  return 0;
}