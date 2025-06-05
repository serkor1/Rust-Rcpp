#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Struct: Vector<T>
///
/// ## Description
///
///
/// C/C++ compatible f64 vectors
///
/// NOTE: if bindgen is C++ this is a struct template
/// otherwise its a typedef (For some reason it becomes vector_f64 if exported like this)
template<typename T>
struct vector {
  const T *data;
  uintptr_t len;
  uintptr_t idx;
};

extern "C" {

double sum(const vector<double> *vector);

}  // extern "C"
