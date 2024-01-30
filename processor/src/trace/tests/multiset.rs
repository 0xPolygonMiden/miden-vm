// TODO: what do we do here?
//  build an aux column such that multiplicand / divisor = 1 even when multiplicands and divisors
//  are not permutations of one another

// TODO:
//  we build the column through the build_aux_column function
//  this takes the main trace and a set of alpha values, also self
//  the multiplicands are gotten from the get_responses_at
//  while divisors are gotten from the get_requests_at

// TODO:
//  Steps:
//   implement some struct that implements AuxColumnBuilder
//      implement get_requests_at, get_responses_at
//   Happy case:
//    call build_aux_column, confirm works
//   Bad case:
//    call build_aux_column, confirm fails
//   Fix running product
//    confirm test passes now
//   Fix other hardcoded tests
//   Look into virtual table comment by plafer
//   Figure out how to update documentation