#include <Rcpp.h>
#include "pkg/api.h"

//' @export
// [[Rcpp::export]]
double reference_sum(const Rcpp::NumericVector& x) {

    const double* data = x.begin();
    int n = x.size();

    double sum = 0.0;
    const double* end = data + n;
    for (const double* ptr = data; ptr < end; ++ptr) {
        sum += *ptr;
    }

    return sum;
}

//' @export
// [[Rcpp::export]]
double ffi_sum(const Rcpp::NumericVector& x) {
    vector<double> slice;
    slice.data = x.begin();
    slice.len  = x.size();
    slice.idx  = 0;
    return sum(&slice);
}

