//
// Created by Elvis Sabanovic on 23/12/2024.
//

// Compute (x + y) mod n

int mod_add(int x, int y, int n) {
  int r = (x + y) % n; // Division is expensive, unless by a power of 2
  // since we do not know wheter n is power of two at compile time, the compiler
  // cannot translate this to right shift operation
  return r;
}

// Unpredictable branch is expensive
// we dont know if z is less than n
int mod_add_branch(int x, int y, int n) {
  int z = x + y;
  int r = (z < n) ? z : z - n;
  return r;
}

// Use branchless min bit-trick
int mod_add_branchless(int x, int y, int n) {
  int z = x + y;
  int r = z - (n & -(z >= n));
  return r;
}

int main() { return 0; }