#include <Rcpp.h>
#include "pkg/api.h"

//' @export
// [[Rcpp::export]]
double rcpp_sum(const std::vector<double>& x) {
    return sum(x.data(), static_cast<uint64_t>(x.size()));
}

//' @export
// [[Rcpp::export]]
double rcpp_add(double x, double y) {
    return add(x, y);
}