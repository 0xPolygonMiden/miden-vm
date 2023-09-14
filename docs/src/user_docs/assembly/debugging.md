# Debugging

To support basic debugging capabilities, Miden assembly provides a `debug` instruction. This instruction prints out the state of the VM at the time when the `debug` instruction is executed. The instruction can be parameterized as follows:

- `debug.stack` prints out the entire contents of the stack.
- `debug.stack.<n>` prints out the top $n$ items of the stack. $n$ must be an integer greater than $0$ and smaller than $256$.

Debug instructions do not affect the VM state and do not change the program hash.

To make use of the `debug` instruction, programs must be compiled with an assembler instantiated in the debug mode. Otherwise, the assembler will simply ignore the `debug` instructions.