#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

/// add two values
double add(double left, double right);

double sum(const double *data, uintptr_t len);

}  // extern "C"
