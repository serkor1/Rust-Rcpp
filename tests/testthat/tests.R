testthat::test_that(
  desc = "Test that functions are working", code = {

    ## test the sum function
    testthat::expect_equal(
      object   = rcpp_sum(c(1.2, 1.3, 2.0)),
      expected = sum(c(1.2, 1.3, 2.0))
    )

    ## test the add function
    testthat::expect_equal(
      object   = rcpp_add(1.2, 1.3),
      expected = sum(c(1.2, 1.3))
    )

    ## test the slicer
    testthat::expect_equal(
      object   = rcpp_sum_slice(c(1.2, 1.3)),
      expected = sum(c(1.2, 1.3))
    )
    
  }
)