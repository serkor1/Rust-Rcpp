#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// C/C++ compatible f64 vectors
template<typename T>
struct RawSlice {
  const T *data;
  uintptr_t len;
  uintptr_t idx;
};

extern "C" {

/// With RawSlice implementations
double sum_slice(const RawSlice<double> *slice);

/// add two values
double add(double left, double right);

double sum(const double *data, uintptr_t len);

}  // extern "C"
