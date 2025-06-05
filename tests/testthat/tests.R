testthat::test_that(
  desc = "Test that functions are working", code = {

    ## test the sum function
    testthat::expect_equal(
      object   = ffi_sum(c(1.2, 1.3, 2.0)),
      expected = sum(c(1.2, 1.3, 2.0))
    )

    ## test the reference sum function
    testthat::expect_equal(
      object   = reference_sum(1.2, 1.3),
      expected = sum(c(1.2, 1.3))
    )
    
  }
)