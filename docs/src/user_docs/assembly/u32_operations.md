## u32 operations
Miden assembly provides a set of instructions which can perform operations on regular 32-bit integers. These instructions are described in the tables below.

Many instructions have *safe* and *unsafe* versions. The safe versions ensure that input values are 32-bit integers, and fail if that's not the case. The *unsafe* versions do not perform these checks, and thus, should be used only if the inputs are known to be 32-bit integers. Supplying inputs which are greater than or equal to $2^{32}$ to *unsafe* operations results in undefined behavior.

The primary benefit of using *unsafe* operations is performance: they can frequently be executed $2$ or $3$ times faster than their safe counterparts. In general, vast majority of the *unsafe* operations listed below can be executed in a single VM cycle.

For instructions where one or more operands can be provided as immediate parameters (e.g., `u32add` and `u32add.b`), we provide stack transition diagrams only for the non-immediate version. For the immediate version, it can be assumed that the operand with the specified name is not present on the stack.

### Conversions and tests

| Instruction   | Stack input | Stack output  | Notes                                      |
| ------------- | ----------- | ------------- | ------------------------------------------ |
| u32test       | [a, ...]    | [b, a, ...]   | $b \leftarrow \begin{cases} 1, & \text{if}\ a < 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ |
| u32testw      | [A, ...]    | [b, A, ...]   | $b \leftarrow \begin{cases} 1, & \text{if}\ \forall\ i \in \{0, 1, 2, 3\}\ a_i < 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ |
| u32assert <br> u32assert.1 | [a, ...]    | [a, ...]  | Fails if $a \ge 2^{32}$ |
| u32assert.2   | [b, a,...]  | [b, a,...] | Fails if $a \ge 2^{32}$ or $b \ge 2^{32}$ |
| u32assertw    | [A, ...]    | [A, ...]      | Fails if $\exists\ i \in \{0, 1, 2, 3\} \ni a_i \ge 2^{32}$ |
| u32cast       | [a, ...]    | [b, ...]      | $b \leftarrow a \mod 2^{32}$ |
| u32split      | [a, ...]    | [c, b, ...]   | $b \leftarrow a \mod 2^{32}$, $c \leftarrow \lfloor{a / 2^{32}}\rfloor$ |

### Arithmetic operations

| Instruction    | Stack_input    | Stack_output  | Notes                                      |
| -------------- | -------------- | ------------- | ------------------------------------------ |
| u32add <br> u32add.*b* | [b, a, ...] | [c, ...] | $c \leftarrow a + b$ <br> Fails if $max(a, b, c) \ge 2^{32}$ |
| u32add.full    | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a + b) \mod 2^{32}$ <br> $d \leftarrow \begin{cases} 1, & \text{if}\ (a + b) \ge 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32add.unsafe  | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a + b) \mod 2^{32}$, <br> $d \leftarrow \begin{cases} 1, & \text{if}\ (a + b) \ge 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32addc        | [b, a, c, ...] | [e, d, ...]   | $d \leftarrow (a + b + c) \mod 2^{32}$ <br> $e \leftarrow \begin{cases} 1, & \text{if}\ (a + b + c) \ge 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ or $c > 1$ |
| u32addc.unsafe | [b, a, c, ...] | [e, d, ...]   | $d \leftarrow (a + b + c) \mod 2^{32}$, <br> $e \leftarrow \begin{cases} 1, & \text{if}\ (a + b + c) \ge 2^{32} \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ <br> Fails if $c > 1$ |
| u32sub <br> u32sub.*b* | [b, a, ...] | [c, ...] | $c \leftarrow (a - b)$ <br> Fails if $max(a, b) \ge 2^{32}$ or $a < b$ |
| u32sub.full    | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a - b) \mod 2^{32}$ <br> $d \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32sub.unsafe  | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a - b) \mod 2^{32}$ <br> $d \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32mul <br> u32mul.*b* | [b, a, ...] | [c, ...] | $c \leftarrow a \cdot b$ <br> Fails if $max(a, b, c) \ge 2^{32}$ |
| u32mul.full    | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a \cdot b) \mod 2^{32}$ <br> $d \leftarrow \lfloor(a \cdot b) / 2^{32}\rfloor$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32mul.unsafe  | [b, a, ...]    | [d, c, ...]   | $c \leftarrow (a \cdot b) \mod 2^{32}$ <br> $d \leftarrow \lfloor(a \cdot b) / 2^{32}\rfloor$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32madd         | [b, a, c, ...] | [e, d, ...]   | $d \leftarrow (a \cdot b + c) \mod 2^{32}$ <br> $e \leftarrow \lfloor(a \cdot b + c) / 2^{32}\rfloor$ <br> Fails if $max(a, b, c) \ge 2^{32}$ |
| u32madd.unsafe  | [b, a, c, ...] | [e, d, ...]   | $d \leftarrow (a \cdot b + c) \mod 2^{32}$ <br> $e \leftarrow \lfloor(a \cdot b + c) / 2^{32}\rfloor$ <br> Undefined if $max(a, b, c) \ge 2^{32}$ |
| u32div <br> u32div.*b* | [b, a, ...] | [c, ...] | $c \leftarrow \lfloor a / b\rfloor$ <br> Fails if $max(a, b) \ge 2^{32}$ or $b = 0$ |
| u32div.full    | [b, a, ...]    | [d, c, ...]   | $c \leftarrow \lfloor a / b\rfloor$ <br> $d \leftarrow a \mod b$ <br> Fails if $max(a, b) \ge 2^{32}$ or $b = 0$ |
| u32div.unsafe  | [b, a, ...]    | [d, c, ...]   | $c \leftarrow \lfloor a / b\rfloor$ <br> $d \leftarrow a \mod b$ <br> Fails if $b = 0$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32mod <br> u32mod.*b* | [b, a, ...] | [c, ...] | $c \leftarrow a \mod b$ <br> Fails if $max(a, b) \ge 2^{32}$ or $b = 0$ |
| u32mod.unsafe  | [b, a, ...]    | [c, ...]      | $c \leftarrow a \mod b$ <br> Fails if $b = 0$ <br> Undefined if $max(a, b) \ge 2^{32}$ |

### Bitwise operations

| Instruction    | Stack_input    | Stack_output  | Notes                                      |
| -------------- | -------------- | ------------- | ------------------------------------------ |
| u32and         | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `AND` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$ |
| u32or          | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `OR` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$ |
| u32xor         | [b, a, ...]    | [c, ...]      | Computes $c$ as a bitwise `XOR` of binary representations of $a$ and $b$. <br> Fails if $max(a,b) \ge 2^{32}$ |
| u32not         | [a, ...]       | [b, ...]      | Computes $b$ as a bitwise `NOT` of binary representation of $a$. <br> Fails if $a \ge 2^{32}$ |
| u32shl <br> u32shl.*b* | [b, a, ...] | [c, ...] | $c \leftarrow (a \cdot 2^b) \mod 2^{32}$ <br> Fails if $a \ge 2^{32}$ or $b > 31$ |
| u32shr <br> u32shr.*b* | [b, a, ...] | [c, ...] | $c \leftarrow \lfloor a/2^b \rfloor$ <br> Fails if $a \ge 2^{32}$ or $b > 31$ |
| u32rotl <br> u32rotl.*b* | [b, a, ...] | [c, ...] | Computes $c$ by rotating a 32-bit representation of $a$ to the left by $b$ bits. <br> Fails if $a \ge 2^{32}$ or $b > 31$ |
| u32rotr <br> u32rotr.*b* | [b, a, ...] | [c, ...] | Computes $c$ by rotating a 32-bit representation of $a$ to the right by $b$ bits. <br> Fails if $a \ge 2^{32}$ or $b > 31$ |

### Comparison operations

| Instruction    | Stack_input  | Stack_output    | Notes                                      |
| -------------- | ------------ | --------------- | ------------------------------------------ |
| u32.eq <br> u32.eq.*b* | [b, a, ...] | [c, ...] | $c \leftarrow \begin{cases} 1, & \text{if}\ a=b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ <br> Note: unsafe version is not provided because it is equivalent to simple `eq`. |
| u32.neq <br> u32.neq.*b* | [b, a, ...] | [c, ...] | $c \leftarrow \begin{cases} 1, & \text{if}\ a \ne b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ <br> Note: unsafe version is not provided because it is equivalent to simple `neq`. |
| u32lt          | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32lt.unsafe   | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a < b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32lte         | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a \le b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32lte.unsafe  | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a \le b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32gt          | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a > b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32gt.unsafe   | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a > b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32gte         | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a \ge b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32gte.unsafe  | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} 1, & \text{if}\ a \ge b \\ 0, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32min         | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} a, & \text{if}\ a < b \\ b, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32min.unsafe  | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} a, & \text{if}\ a < b \\ b, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
| u32max         | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} a, & \text{if}\ a > b \\ b, & \text{otherwise}\ \end{cases}$ <br> Fails if $max(a, b) \ge 2^{32}$ |
| u32max.unsafe  | [b, a, ...] | [c, ...]         | $c \leftarrow \begin{cases} a, & \text{if}\ a > b \\ b, & \text{otherwise}\ \end{cases}$ <br> Undefined if $max(a, b) \ge 2^{32}$ |
