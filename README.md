<!-- badges: start -->
[![R-CMD-check](https://github.com/serkor1/Rust-Rcpp/actions/workflows/R-CMD-check.yaml/badge.svghttps://github.com/serkor1/Rust-Rcpp/actions/workflows/R-CMD-check.yaml/badge.svghttps://github.com/serkor1/Rust-Rcpp/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/serkor1/Rust-Rcpp/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

# Using Rust in R via [{Rcpp}](https://github.com/RcppCore/Rcpp/tree/master)

This guide is written by someone who doesn't *really* know `Rust`, `C++` or `C` above and beyond the "basics" - so it might not be the best place to look for guidance. Its primarily written for my own understanding and create R bindings for [perpetual](https://github.com/perpetual-ml/perpetual).

Most of the stuff here is "borrowed" from [here](https://github.com/r-rust/hellorust) which is the work of @jeroen.

Why not use [{extendr}](https://github.com/extendr/rextendr)? Well, compare its dependencies to that of [{Rcpp}](https://github.com/RcppCore/Rcpp/tree/master) - regardless of how awesome the idea is, its basically bloatware.

Why not use `C` and [R internals](https://cran.r-project.org/doc/manuals/r-release/R-ints.html)? Well, if you know how to use it properly, use it. I can't - but [{Rcpp}](https://github.com/RcppCore/Rcpp/tree/master) does all the dirty work for me, so I don't have to use `C`.

## Repository structure

```console
.
├── Makefile
├── NAMESPACE
├── R
│   ├── RcppExports.R
│   └── RustPkg-package.R
├── README.md
└── src
    ├── Makevars
    ├── pkg // This is where the Rust project lives.
    │   ├── api.h
    │   ├── Cargo.lock
    │   ├── Cargo.toml
    │   ├── src
    │   │   └── lib.rs
    │   ├── vendor-authors.R
    │   ├── vendor-config.toml
    │   └── vendor-update.sh
    ├── RcppExports.cpp
    └── wrapper.cpp
```

## :books: Where do I start?

From project root:

```console
make help
```

Is a good start.

## :books: Resources

* On [configure](https://yutani.rbind.io/post/2021-09-21-writing-a-configure-script-for-an-r-package-using-rust/?utm_source=chatgpt.com) scripts. 
