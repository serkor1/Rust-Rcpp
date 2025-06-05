## script: Benchmark
## objective: Calculate overhead
## of Rust call relative to {Rcpp}
rm(list = ls()); gc();

## 1) define vector
x <- rnorm(n = 1e8);

bench::mark(
  `{Rust}` = RustPkg::ffi_sum(x),
  `{Rcpp}` = RustPkg::reference_sum(x),
  `{R}`    = sum(x)
)

