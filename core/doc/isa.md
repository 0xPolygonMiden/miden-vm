# Miden VM instruction set
Miden VM instruction set consists of a small number of atomic instructions. There are two types of instructions:

* **System instructions** are encoded with a 3-bit opcode. They are used to control program execution path and are executed automatically by the VM as it traverses [program execution graph](programs.md).
* **User instructions** are encoded with a 7-bit opcode. A sequence of user instructions forms a an [instruction block](programs.md/#Instruction-blocks) in a program execution graph.

In every cycle, Miden VM executes a tuple of two instructions: one system instruction and one user instruction. However, not all combinations of system and user instructions are valid, and specifics of this are described in the following sections.

Miden VM consumes programs in a form of an execution graph. An execution graph can be constructed directly from program blocks, but constructing programs in this manner may be tedious and error-prone. So, most **users are encouraged** to write programs using [Miden assembly](../../assembly/doc/assembly.md) instead. However, it is still beneficial to understand which raw instructions are available in Miden VM and what their semantics are.

The tables below describe all currently available atomic instructions in Miden VM.

## System instructions

| Instruction | Opcode   | Description                             |
| ----------- | :------: | --------------------------------------- |
| HACC        | 000      | Indicates that any user instruction can be executed on the VM. |
| BEGIN       | 001      | Marks the beginning of a new Group or Switch block; can be executed only on steps which are one less than a multiple of 16 (e.g. 15, 31, 47 etc.). |
| TEND        | 010      | Marks the end of a Group block, a true branch of a Switch block, or a Loop block when loop body was executed at least once; can be executed on steps which are multiples of 16 (e.g. 16, 32, 48 etc.). |
| FEND        | 011      | Marks the end of a false branch of a Switch block, or a skip path of a Loop block when a loop was never entered; can be executed on steps which are multiples of 16 (e.g. 16, 32, 48 etc.). |
| LOOP        | 100      | Marks the beginning of a new Loop block; can be executed only on steps which are one less than a multiple of 16. |
| WRAP        | 101      | Indicates that a currently executing loop should be executed for at least one more iteration; can be executed only on steps which are one less than a multiple of 16. |
| BREAK       | 110      | Indicates that a currently executing loop should be exited; can be executed only on steps which are one less than a multiple of 16. |
| VOID        | 111      | Pads program execution so that number of executed cycles is equal to 2<sup>n</sup> for some integer *n*. A `VOID` instruction can be followed only by another `VOID` instruction. Thus, once a single `VOID` instruction is executed, no other instruction can be executed on the VM. All programs must end with a `VOID` instruction. |

## User instructions

User instructions can be executed only concurrently with the `HACC` system instruction. The only exception is the `NOOP` instruction, which can be executed concurrently with any of the system instructions.

### Flow control instructions

| Instruction | Opcode   | Description                             |
| ----------- | :------: | --------------------------------------- |
| BEGIN       |  0000000 | Marks the beginning of a program. Every program must start with the `BEGIN` operation. If executed on any step but the first one, the operation will fail.|
| NOOP        |  1111111 | Does nothing. |
| ASSERT      |  1100000 | Pops the top item from the stack and checks if it is equal to `1`. If it is not equal to `1`, the operation will fail. |
| ASSERTEQ    |  1100001 | Pops top two items from the stack and checks if they are equal. If they are not equal, the operation will fail. |

### Input instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| PUSH        |  0011111 | Pushes a 128-bit value (a single field element) onto the stack. |
| READ        |  1110000 | Pushes the next value from the input tape `A` onto the stack. |
| READ2       |  1110001 | Pushes the next values from input tapes `A` and `B` onto the stack. Value from input tape `A` is pushed first, followed by the value from input tape `B`. |

### Stack manipulation instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| DUP         |  1110010 | Pushes a copy of the top stack item onto the stack (duplicates the top stack item). |
| DUP2        |  1110011 | Pushes copies of the top two stack items onto the stack. |
| DUP4        |  1110100 | Pushes copies of the top four stack items onto the stack. |
| PAD2        |  1110101 | Pushes two `0` values onto the stack. Equivalent to `PUSH(0) DUP`. |
| DROP        |  1100011 | Removes the top item from the stack. |
| DROP4       |  1100100 | Removes top four items from the stack. |
| SWAP        |  1111000 | Moves the second from the top stack item to the top of the stack (swaps top two stack items). |
| SWAP2       |  1111001 | Moves 3rd and 4th stack items to the top of the stack. For example, assuming `S0` is the top of the stack, `S0 S1 S2 S3` becomes `S2 S3 S0 S1`. |
| CSWAP2      |  1100111 | If the 5th stack item is `1`, swaps top 2 stack items (similar to `SWAP2` instructions); if the 5th stack item is `0`, the top 4 stack items remain unchanged; otherwise the operation fails. Stack items 5 and 6 are discarded. |
| SWAP4       |  1111010 | Moves 5th through 8th stack items to the top of the stack. For example, assuming `S0` is the top of the stack, `S0 S1 S2 S3 S4 S5 S6 S7` becomes `S4 S5 S6 S7 S0 S1 S2 S3`. |
| ROLL4       |  1111011 | Moves 4th stack item to the top of the stack. For example, assuming `S0` is the top of the stack, `S0 S1 S2 S3` becomes `S3 S0 S1 S2`.  |
| ROLL8       |  1111100 | Moves 8th stack item to the top of the stack. For example, assuming `S0` is the top of the stack, `S0 S1 S2 S3 S4 S5 S6 S7` becomes `S7 S0 S1 S2 S3 S4 S5 S6`. |

### Arithmetic and boolean instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| ADD         |  1101000 | Pops top two items from the stack, adds them, and pushes the result onto the stack. |
| MUL         |  1101001 | Pops top two items from the stack, multiplies them, and pushes the result onto the stack. |
| AND         |  1101010 | Pops top two items from the stack, computes an equivalent of their boolean `AND` (which, for binary values, is just multiplication), and pushes the result onto the stack. If either of the values is not binary, the operation will fail. |
| OR          |  1101011 | Pops top two items from the stack, computes an equivalent of their boolean `OR`, and pushes the result onto the stack. If either of the values is not binary, the operation will fail. |
| INV         |  1101100 | Pops the top item from the stack, computes its multiplicative inverse, and pushes the result onto the stack. This can be used to emulate division with a sequence of two operations: `INV MUL`. If the value at the top of the stack is `0`, the operation will fail.
| NEG         |  1101101 | Pops the top item from the stack, computes its additive inverse, and pushes the result onto the stack. This can be used to emulate subtraction with a sequence of two operations: `NEG ADD` |
| NOT         |  1101110 | Pops the top item from the stack, subtracts it from value `1` and pushes the result onto the stack. In other words, `0` becomes `1`, and `1` becomes `0`. This is equivalent to `PUSH 1 SWAP NEG ADD` but also enforces that the top stack item is a binary value. |

### Comparison instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| EQ          |  1100010 | Pops top 3 values from the stack, subtracts the 3rd value from the 2nd, then multiplies the result by the 1st value, and then subtracts the result from value `1` and pushes the final result onto the stack. The operation can be used to check whether two values are equal (see [here](#Checking-equality)). |
| CMP         |  0111111 | Pops top 8 items from the top of the stack, performs a single round of binary comparison, and pushes the resulting 8 values onto the stack. This operation can be used as a building block for *less then* and *greater than* operations (see [here](#Checking-inequality)). |
| BINACC      |  1111101 | Pops top 4 items from the top of the stack, performs a single round of binary aggregation, and pushes the resulting 4 values onto the stack. This operation can be used as a building block for range check operations (see [here](#Checking-binary-decomposition)). |

### Selection instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| CHOOSE      |  1100101 | Pops 3 items from the top of the stack, and pushes either the 1st or the 2nd value back onto the stack depending on whether the 3rd value is `1` or `0`. For example, assuming `S0` is the top of the stack, `S0 S1 1` becomes `S0`, while `S0 S1 0` becomes `S1`. This operation will fail if the 3rd stack item is not a binary value. |
| CHOOSE2     |  1100110 | Pops 6 items from the top of the stack, and pushes either the 1st or the 2nd pair of values back onto the stack depending on whether the 5th value is `1` or `0`. For example, assuming `S0` is the top of the stack, `S0 S1 S2 S3 1 S5` becomes `S0 S1`, while `S0 S1 S2 S3 0 S5` becomes `S2 S3` (notice that `S5` is discarded in both cases). This operation will fail if the 5th stack item is not a binary value. |

### Cryptographic instructions

| Instruction | Opcode   | Description                            |
| ----------- | :------: | -------------------------------------- |
| RESCR       |  1011111 | Pops top 6 items from the stack, computes a single round of a modified [Rescue](https://eprint.iacr.org/2019/426) hash function over these values, and pushes the resulting 6 values onto the stack. This operation can be used to hash up to two 256-bit values (see [here](#Hashing-in-Miden-VM)).  |

## Value comparison in Miden VM
There are 3 operations in Miden VM which can be used to compare values: `EQ`, `CMP`, and `BINACC`. Using these operations you can check whether 2 values a equal, whether one value is greater or less than the other, and whether a value can be represented with a given number of bits.

### Checking equality
Using `EQ` operation you can determine whether two values are equal. Before executing this operation, you should position values on the stack in the appropriate order. Specifically, the stack should look like so:
```
[inv_dif, x, y]
```
where:
* `x` and `y` are the values you want to compare;
* `inv_dif` is equal to `inv(x - y)` when `x != y`, and to any value otherwise. If `inv_dif` value does not satisfy these conditions, the operation will fail.

Once the stack has been arranged as described above, executing `EQ` operation, will do the following:
1. Pop top 3 values from the stack;
2. Push `1` onto the stack if `x == y`, and push `0` onto the stack otherwise.

`inv_dif` value should be computed beforehand and provided to the VM via input tape `A`. In this way, checking equality between the top two stack items can be accomplished by the following sequence of instructions:
```
READ EQ
```

### Checking inequality
Using repeated execution of `CMP` operation you can determine if one value is greater or less than another value. Executing this operation consumes a single input from each of the input tapes. It also assumes that you've positioned items on the stack in an appropriate order. If items on the stack are not positioned correctly, the result of the operation will be undefined.

Supposed we wanted to compare 2 values: `a` and `b` (both are 128-bit field elements). To accomplish this, we'd need to position elements on the stack like so:

```
[p, 0, 0, 0, 0, 0, 0, 0, a, b]
```
where `p` = 2<sup>n - 1</sup> for some `n` <= 128 such that 2<sup>n</sup> > `a`, `b`. For example, if `a` and `b` are unconstrained field elements, `p` should be set to 2<sup>127</sup>. Or, if `a` and `b` are know to be 64-bit numbers, `p` should be set to 2<sup>63</sup>.

Once the stack has been arranged in this way, we'll need to execute `CMP` operation `n` times in a row. As mentioned above, each execution of the operation consumes inputs from tapes `A` and `B`. The tapes must be populated with binary representations of values `a` and `b` respectively (in [big-endian](https://en.wikipedia.org/wiki/Endianness) order). For example, if `a = 5` and `b = 8`, input tape `A` should be `[0, 1, 0, 1]`, and input tape `B` should be `[1, 0, 0, 0]`.

After we execute `CMP` operation `n` number of times, the stack will have the following form:
```
[x, x, x, x, gt, lt, b_acc, a_acc, a, b]
```
where:
* `x` values are intermediate results of executing `CMP` operations and should be discarded.
* `gt` value will be `1` if `a` > `b`, and `0` otherwise.
* `lt` value will be `1` if `a` < `b`, and `0` otherwise.
* `a_acc` will be equal to the result of aggregating value `a` from its binary representation.
* `b_acc` will be equal to the result of aggregating value `b` from its binary representation.

To make sure that the comparison is valid, we need to check that `a` == `a_acc` and `b` == `b_acc`. If these checks pass, then both numbers can be represented by `n`-bit values. This, in turn, means that the comparison is valid. The instruction sequences below can be executed after the last `CMP` operation in the sequence to perform these comparisons and remove un-needed values from the stack:

```
// performs the comparisons and leaves only the lt value on the stack
DROP4 PAD2 SWAP4 ROLL4 ASSERTEQ ASSERTEQ DUP DROP4

// performs the comparisons and leaves only the gt value on the stack
DROP4 PAD2 SWAP4 ROLL4 ASSERTEQ ASSERTEQ ROLL4 DUP DROP4
```

Overall, the number of operations needed to compare 2 values is proportional to the size of the values. Specifically:

* Comparing two unconstrained field elements requires ~ 140 operations,
* Comparing two 64-bit values requires ~ 75 operations,
* Comparing two 32-bit values requires ~ 45 operations.

### Checking binary decomposition
Sometimes it may be useful to check whether a value fits into a certain number of bits. This can be accomplished with `CMP` operations, but `BINACC` operation provides a simpler way to do this.

Similar to `CMP` operation, `BINACC` operation needs to be executed `n` times in a row if we want to make sure that a value can be represented with `n` bits.

Each execution of the operation consumes a single input from tape `A`. The tape must be populated with binary representation of value `a` in [little-endian](https://en.wikipedia.org/wiki/Endianness) order. For example, if `a = 5`, input tape `A` should be `[1, 0, 1, 0]`.

Also similar to `CMP` operation, `BINACC` operation expect items on the stack to be arranged in a certain order. If the items are not arranged as shown below, the result of the operation is undefined:

```
[0, 0, 1, 0, a]
```
where `a` is the value we want to range-check,

Once items on the stack have been arranged as described above, we execute `BINACC` instruction `n` times. This will leave the stack in the following form:
```
[x, x, x, a_acc, a]
```
where:
* `x` values are intermediate results of executing `BINACC` operations and should be discarded.
* `a_acc` will be equal the result of aggregating value `a` from its binary representation.

To make sure that `a` can fit into `n` bits we need to check that `a_acc = a`. This can be done using the following sequence of operations:
```
DUP DROP4 READ EQ
```
The above sequence discards the first three items, then checks the equality of the remaining two items as described [here](#Checking-equality) placing `1` onto the stack if the values are equal, and `0` otherwise.

Overall, the number of operations needed to determine whether a value can be represented by `n` bits is `n + 4` (assuming you already have the value positioned correctly on the stack). Specifically:

* Checking if a value can be represented with 64 bits requires 68 operations,
* Checking if a value can be represented with 32 bits requires 36 operations.

## Hashing in Miden VM
Miden VM provides a `RESCR` instruction which can be used as a building block for computing cryptographic hashes. The `RESCR` instruction computes a single round of a modified [Rescue hash function](https://eprint.iacr.org/2019/426) over the top 6 items of the stack. Specifically, the top 6 stack items form the state of the sponge with the items at the top of the stack considered to be the inner part of the sponge, while the items at the bottom of the stack are considered to be the outer part of the sponge.

The pseudo-code for the modified Rescue round looks like so:
```
add round constants;
apply s-box of power 3;
apply MDS;
add round constants;
apply inverse s-box;
apply MDS;
```
This modification makes the arithmetization of the function fully foldable. It should not impact security properties of the function, but it is worth noting that this has not been studied to the same extent as the standard Rescue hash function.

Another thing to note is that round constants are on a cycle that repeats every 16 steps. This makes `RESCR` operation context-dependant. Meaning, executing `RESCR` operation on step 1 will use different round constant as compared to executing the operation on step 2. But `RESCR` on step 1 will use the same constants as `RESCR` on step 17.

### Using RESCR instruction

Generally, we want to hash values that are 256 bits long. And since all values in Miden VM are elements of a 128-bits field, we'll need 2 elements to represent each 256-bit value. For example, suppose we wanted to compute `hash(x)`. First, we'd represent `x` by a pair of elements `(x0, x1)`, and then we'd position these elements on the stack like so:
```
[0, 0, 0, 0, x1, x0]
```
In other words, the first 4 items of the stack (the inner part of the sponge) should be set to `0`'s, and the following 2 items (the outer part of the sponge) should be set to the elements representing the value we want to hash.

If we wanted to compute a hash of two values `hash(x, y)`, represented by elements `(x0, x1)` and `(y0, y1)` respectively, we'd position them on the stack like so:
```
[0, 0, y1, y0, x1, x0]
```
After the stack is set up for hashing, we execute `RESCR` operation multiple times and then remove the inner part of the sponge from the stack. This way, the outer part of the sponge remains at the top of the stack.

There are a few things to keep in mind:

1. To achieve adequate security (e.g. 120-bits), `RESCR` operation must be executed at least 10 times in a row.
2. Since `RESCR` operation is context-dependant, the first `RESCR` operation in every sequence must be executed on the step which is a multiple of 16 (e.g. 16, 32, 48 etc.). To ensure this alignment, you can always use `NOOP` operations to pad your programs.
3. The inner part of the sponge must consist of at least 2 elements. That is, when setting the stack up for hashing, the top 2 items of the stack must always be set to `0`'s.

Below is an example of a program which reads two 256-bit values from input tape `A` and computes their hash:
```
BEGIN NOOP  NOOP  NOOP  NOOP  NOOP  NOOP  NOOP
NOOP  NOOP  NOOP  READ  READ  READ  READ  PAD2
RESCR RESCR RESCR RESCR RESCR RESCR RESCR RESCR
RESCR RESCR DROP4
```
A quick explanation of what's happening here:
1. First, we pad the beginning of the program with `NOOP`'s so that the first `RESCR` operation happens on the 16th step.
2. Then, we read 4 values from the input tape `A` using four `READ` operations. These 4 values represent our two 256-bit values.
3. Then, we push two `0`'s onto the stack to initialize the capacity portion of the sponge. This is done by executing `PAD2` operation.
4. Then, we execute `RESCR` operation 10 times. Notice again that the first `RESCR` operation is executed on the 16th step.
5. The result of hashing is now in the 5th and 6th positions of the stack. So, we remove top 4 times from the stack (using `DROP4` operation) to move the result to the top of the stack.