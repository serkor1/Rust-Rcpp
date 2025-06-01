#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// C/C++ compatible f64 vectors
struct F64Slice {
  const double *data;
  uintptr_t len;
};

extern "C" {

/// With F64Slice implementations
double sum_slice(const F64Slice *slice);

/// add two values
double add(double left, double right);

double sum(const double *data, uintptr_t len);

}  // extern "C"
