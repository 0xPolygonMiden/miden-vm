## u32 operations
Miden assembly provides a set of instructions which can perform operations on regular two-complement 32-bit integers. These instructions are described in the tables below.

For instructions where one or more operands can be provided as immediate parameters (e.g., `u32wrapping_add` and `u32wrapping_add.b`), we provide stack transition diagrams only for the non-immediate version. For the immediate version, it can be assumed that the operand with the specified name is not present on the stack.

In all the table below, the number of cycles it takes for the VM to execute each instruction is listed beneath the instruction.

### Conversions and tests

| Instruction                                    | Stack input | Stack output  | Notes                                                                                                                          |
| ---------------------------------------------- | ----------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| u32test <br> - *(5 cycles)*                    | [a, ...]    | [b, a, ...]   | $b \leftarrow \begin{cases} 1, & \text{if}\ a < 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$                                  |
| u32testw <br> - *(23 cycles)*                  | [A, ...]    | [b, A, ...]   | $b \leftarrow \begin{cases} 1, & \text{if}\ \forall\ i \in \{0, 1, 2, 3\}\ a_i < 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ |
| u32assert <br> - *(3 cycles)* | [a, ...]    | [a, ...]      | Fails if $a \ge 2^{32}$                                                                                                        |
| u32assert2 <br> - *(1 cycle)*                 | [b, a,...]  | [b, a,...]    | Fails if $a \ge 2^{32}$ or $b \ge 2^{32}$                                                                                      |
| u32assertw <br> - *(6 cycles)*                 | [A, ...]    | [A, ...]      | Fails if $\exists\ i \in \{0, 1, 2, 3\} : a_i \ge 2^{32}$                                                                    |
| u32cast <br> - *(2 cycles)*                    | [a, ...]    | [b, ...]      | $b \leftarrow a \mod 2^{32}$                                                                                                   |
| u32split <br> - *(1 cycle)*                    | [a, ...]    | [c, b, ...]   | $b \leftarrow a \mod 2^{32}$, $c \leftarrow \lfloor{a / 2^{32}}\rfloor$                                                        |

The instructions `u32assert`, `u32assert2` and `u32assertw` can also be parametrized with an error code which can be any 32-bit value specified either directly or via a [named constant](./code_organization.md#constants). For example:
```
u32assert.err=123
u32assert.err=MY_CONSTANT
```
If the error code is omitted, the default value of $0$ is assumed.

### Arithmetic operations

| Instruction                                                                               | Stack input    | Stack output  | Notes                                                                                                                                                                                  |
| ----------------------------------------------------------------------------------------- | -------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| u32overflowing_add <br> - *(1 cycle)* <br> u32overflowing_add.*b* <br> - *(2-3 cycles)*   | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a + b) \mod 2^{32}$ <br> $d \leftarrow \begin{cases} 1, & \text{if}\ (a + b) \ge 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32wrapping_add <br> - *(2 cycles)* <br> u32wrapping_add.*b* <br> - *(3-4 cycles)*        | [b, a, ...]    | [c, ...]      | $c \leftarrow (a + b) \mod 2^{32}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                                            |
| u32overflowing_add3 <br> - *(1 cycle)*                                                    | [c, b, a, ...] | [e, d, ...]   | $d \leftarrow (a + b + c) \mod 2^{32}$, <br> $e \leftarrow \lfloor (a + b + c) / 2^{32}\rfloor$ <br> Undefined if $max(a, b, c) \ge 2^{32}$ <br>                                       |
| u32wrapping_add3 <br> - *(2 cycles)*                                                      | [c, b, a, ...] | [d, ...]      | $d \leftarrow (a + b + c) \mod 2^{32}$, <br> Undefined if $max(a, b, c) \ge 2^{32}$ <br>                                                                                               |
| u32overflowing_sub <br> - *(1 cycle)* <br> u32overflowing_sub.*b* <br> - *(2-3 cycles)*   | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a - b) \mod 2^{32}$ <br> $d \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$              |
| u32wrapping_sub <br> - *(2 cycles)* <br> u32wrapping_sub.*b* <br> - *(3-4 cycles)*        | [b, a, ...]    | [c, ...]      | $c \leftarrow (a - b) \mod 2^{32}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                                            |
| u32overflowing_mul <br> - *(1 cycle)* <br> u32overflowing_mul.*b* <br> - *(2-3 cycles)*   | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a \cdot b) \mod 2^{32}$ <br> $d \leftarrow \lfloor(a \cdot b) / 2^{32}\rfloor$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                 |
| u32wrapping_mul <br> - *(2 cycles)* <br> u32wrapping_mul.*b* <br> - *(3-4 cycles)*        | [b, a, ...]    | [c, ...]      | $c \leftarrow (a \cdot b) \mod 2^{32}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                                        |
| u32overflowing_madd <br> - *(1 cycle)*                                                    | [b, a, c, ...] | [e, d, ...]   | $d \leftarrow (a \cdot b + c) \mod 2^{32}$ <br> $e \leftarrow \lfloor(a \cdot b + c) / 2^{32}\rfloor$ <br> Undefined if $max(a, b, c) \ge 2^{32}$                                      |
| u32wrapping_madd <br> - *(2 cycles)*                                                      | [b, a, c, ...] | [d, ...]      | $d \leftarrow (a \cdot b + c) \mod 2^{32}$ <br> Undefined if $max(a, b, c) \ge 2^{32}$                                                                                                 |
| u32div <br> - *(2 cycles)* <br> u32div.*b* <br> - *(3-4 cycles)*      | [b, a, ...]    | [c, ...]      | $c \leftarrow \lfloor a / b\rfloor$ <br> Fails if $b = 0$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                     |
| u32mod <br> - *(3 cycles)* <br> u32mod.*b* <br> - *(4-5 cycles)*      | [b, a, ...]    | [c, ...]      | $c \leftarrow a \mod b$ <br> Fails if $b = 0$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                                 |
| u32divmod <br> - *(1 cycle)* <br> u32divmod.*b* <br> - *(2-3 cycles)* | [b, a, ...]    | [d, c, ...]   | $c \leftarrow \lfloor a / b\rfloor$ <br> $d \leftarrow a \mod b$ <br> Fails if $b = 0$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                        |

### Bitwise operations

| Instruction                                                                           | Stack input    | Stack output  | Notes                                                                                                                          |
| ------------------------------------------------------------------------------------- | -------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| u32and <br> - *(1 cycle)*                                                     | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `AND` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$                  |
| u32or <br> - *(6 cycle)s*                                                     | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `OR` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$                   |
| u32xor <br> - *(1 cycle)*                                                     | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `XOR` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$                  |
| u32not <br> - *(5 cycles)*                                                    | [a, ...]       | [b, ...]      | Computes $b$ as a bitwise `NOT` of binary representation of $a$. <br> Fails if $a \ge 2^{32}$                                  |
| u32shl <br> - *(40 cycles)* <br> u32shl.*b* <br> - *(3 cycles)*   | [b, a, ...]    | [c, ...]      | $c \leftarrow (a \cdot 2^b) \mod 2^{32}$ <br> Undefined if $a \ge 2^{32}$ or $b > 31$                                          |
| u32shr <br> - *(40 cycles)* <br> u32shr.*b* <br> - *(3 cycles)*   | [b, a, ...]    | [c, ...]      | $c \leftarrow \lfloor a/2^b \rfloor$ <br> Undefined if $a \ge 2^{32}$ or $b > 31$                                              |
| u32rotl <br> - *(40 cycles)* <br> u32rotl.*b* <br> - *(3 cycles)* | [b, a, ...]    | [c, ...]      | Computes $c$ by rotating a 32-bit representation of $a$ to the left by $b$ bits. <br> Undefined if $a \ge 2^{32}$ or $b > 31$  |
| u32rotr <br> - *(44 cycles)* <br> u32rotr.*b* <br> - *(3 cycles)* | [b, a, ...]    | [c, ...]      | Computes $c$ by rotating a 32-bit representation of $a$ to the right by $b$ bits. <br> Undefined if $a \ge 2^{32}$ or $b > 31$ |
| u32popcnt <br> - *(33 cycles)*                                              | [a, ...]       | [b, ...]      | Computes $b$ by counting the number of set bits in $a$ (hamming weight of $a$). <br> Undefined if $a \ge 2^{32}$               |

### Comparison operations

| Instruction                                                                      | Stack input  | Stack output    | Notes                                                                                                                                                                                                                  |
| -------------------------------------------------------------------------------- | ------------ | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| u32lt <br> - *(5 cycles)*                                              | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                      |
| u32lte <br> - *(7 cycles)*                                             | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} 1, & \text{if}\ a \le b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                    |
| u32gt <br> - *(6 cycles)*                                              | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} 1, & \text{if}\ a > b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                      |
| u32gte <br> - *(6 cycles)*                                             | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} 1, & \text{if}\ a \ge b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                    |
| u32min <br> - *(8 cycles)*                                             | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} a, & \text{if}\ a < b \\ b, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                      |
| u32max <br> - *(9 cycles)*                                             | [b, a, ...]  | [c, ...]        | $c \leftarrow \begin{cases} a, & \text{if}\ a > b \\ b, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$                                                                                      |
