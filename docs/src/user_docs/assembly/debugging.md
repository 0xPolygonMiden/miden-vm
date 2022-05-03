## Debugging
To support basic debugging capabilities, Miden assembly provides `debug` instruction. This instruction prints out the state of the VM at the time when the `debug` instruction is executed. Debug instruction can be parameterized as follows:

- `debug.all` prints out the entire state of the VM (stack and RAM).
- `debug.stack` prints out the entire contents of the stack.
- `debug.stack.<n>` prints out the top $n$ items of the stack. $n$ must be an integer greater than $0$.
- `debug.mem` prints out the entire contents of RAM.
- `debug.mem.<n>` prints out contents of memory at address $n$.
- `debug.mem.<n>.<m>` prints out the contents of memory starting at address $n$ and ending at address $m$ (both inclusive). $m$ must be greater than $n$.
- `debug.local` prints out all locals of the currently executing procedure.
- `debug.local.<n>` prints out contents of the local at index $n$ for the currently executing procedure.

Debug instructions do not affect VM state, do not change program hash, and are omitted when Miden assembly is serialized into binary format.