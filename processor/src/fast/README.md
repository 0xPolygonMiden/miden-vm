# Current conclusions
- We should use an array `[Felt; N]` instead of a `Vec<Felt>` for the stack
    - Gives us a ~30% performance boost (387 MHz -> 571 MHz)
- Using unchecked operations provides marginal benefit, and isn't worth worrying about
    - In one benchmark, it even reduced the performance slightly
- Use macro-ops
    - the big performance hit is in introducing a `match` in the first place; not so much the number of variants in the match statement
- Don't worry about optimizing the `MastForest` struct

# Rough performance outline

- Hardcode the program, use array as stack: **3.1 GHz**
- hardcode all of fibonacci (version 2): **2.5 GHz**
    - Turns out that writing out "swap dup.1 add" in succession speeds the program up (version 1)
- Don't hardcode the program, use dummy `Operation` enum, use array as stack: **590 MHz**
    - Not knowing the program in advance results in a 4-5x performance drop
- Don't hardcode the program, use dummy `Operation` enum, use `Vec` as stack: **385 MHz**
    - Performance drops by ~30% when we switch from an array to a `Vec`
- Don't hardcode the program, use *real* `Operation` enum, use `Vec` as stack: **375 MHz**
    - Switching to actually executing the `MastForest` results in a ~2% performance drop compared to the dummy `Operation` enum
- Don't hardcode the program, use *real* `Operation` enum, use array as stack: **405 MHz**
- Same as last, but use a `swap dup.1 add` macro-op: **760 MHz**
