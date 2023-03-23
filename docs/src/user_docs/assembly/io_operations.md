## Input / output operations
Miden assembly provides a set of instructions for moving data between the stack and several other sources. These sources include:

* **Program code**: values to be moved onto the stack can be hard-coded in a program's source code.
* **Environment**: values can be moved onto the stack from environment variables. Currently, the available environment variables are *stack_depth*, which holds the current depth of the stack, and *local_address*, which stores absolute addresses of local variables. In the future, other environment variables may be added.
* **Advice stack**: values can be moved onto the stack from the advice provider by popping them from the advice stack. There is no limit on the number of values in the advice stack.
* **Memory**: values can be moved between the stack and random-access memory. The memory is word-addressable, meaning, four elements are located at each address, and we can read and write elements to/from memory in batches of four. Memory can be accessed via absolute memory references (i.e., via memory addresses) as well as via local procedure references (i.e., local index). The latter approach ensures that a procedure does not access locals of another procedure.

In the future several other sources such as *storage* and *logs* may be added.

### Constant inputs

| Instruction     | Stack_input | Stack_output | Notes                                      |
| --------------- | ----------- | ------------ | ------------------------------------------ |
| push.*a* <br> - *(1-2 cycles)* <br> push.*a*.*b* <br> push.*a*.*b*.*c*... | [ ... ] | [a, ... ] <br> [b, a, ... ] <br> [c, b, a, ... ] | Pushes values $a$, $b$, $c$ etc. onto the stack. Up to $16$ values can be specified. All values must be valid field elements in decimal (e.g., $123$) or hexadecimal (e.g., $0x7b$) representation. |

When specifying values in hexadecimal format, it is possible to omit the periods between individual values as long as total number of specified bytes is a multiple of $8$. That is, the following are semantically equivalent:

```
push.0x1234.0xabcd
push.0x0000000000001234000000000000abcd
```
In both case the values must still encode valid field elements.

### Environment inputs

| Instruction     | Stack_input | Stack_output | Notes                                      |
| --------------- | ----------- | ------------ | ------------------------------------------ |
| sdepth <br> - *(1 cycle)*        | [ ... ] | [d, ... ] | $d \leftarrow stack.depth()$ <br> Pushes the current depth of the stack onto the stack. |
| caller <br> - *(1 cycle)*        | [ A, b, ... ] | [H, b, ... ] | $H \leftarrow context.fn\_hash()$ <br> Overwrites the top four stack items with the hash of a function which initiated the current SYSCALL. <br> Executing this instruction outside of SYSCALL context will fail. |
| locaddr.*i* <br> - *(2 cycles)*  | [ ... ] | [a, ... ] | $a \leftarrow address\_of(i)$ <br> Pushes the absolute memory address of local memory at index $i$ onto the stack. |
| clk <br> - *(1 cycle)*           | [ ... ] | [t, ... ] | $t \leftarrow clock\_value()$ <br> Pushes the current value of the clock cycle counter onto the stack. |

### Non-deterministic inputs

| Instruction     | Stack_input | Stack_output | Notes                                      |
| --------------- | ----------- | ------------ | ------------------------------------------ |
| adv_push.*n* <br> - *(n cycles)*   | [ ... ]         | [a, ... ]    | $a \leftarrow stack.pop()$ <br> Pops $n$ values from the advice stack and pushes them onto the operand stack. Valid for $n \in \{1, ..., 16\}$. <br> Fails if the advice stack has fewer than $n$ values. |
| adv_loadw <br> - *(1 cycle)*     | [0, 0, 0, 0, ... ] | [A, ... ] | $A \leftarrow stack.pop(4)$ <br> Pop the next word (4 elements) from the advice stack and overwrites the first word of the operand stack (4 elements) with them. <br> Fails if the advice stack has fewer than $4$ values. |
| adv_pipe <br> - *(2 cycles)*     | [S2, S1, S0, a, ... ] | [T2, T1, T0, b, ... ] | $[T_0, T_1, T_2] \leftarrow permute(S_0, stack.pop(4), stack.pop(4))$ <br> $b \leftarrow a + 2$ <br> Pops the next two words (8 elements) from the advice stack, inserts them into memory at address $a$ sequentially, overwrites these top 8 elements onto the operand stack, and performs a Rescue Prime Optimized permutation to the top 12 elements of the operand stack. At the end of the operation, the address is incremented by $2$. <br> Fails if the advice stack has fewer than $8$ values. |

> **Note**: The opcodes above always push data onto the operand stack so that the first element is placed deepest in the stack. For example, if the data on the stack is `a,b,c,d` and you use the opcode `adv_push.4`, the data will be `d,c,b,a` on your stack. This is also the behavior of the other opcodes.

### Random access memory

 As mentioned above, there are two ways to access memory in Miden VM. The first way is via memory addresses using the instructions listed below. The addresses are absolute - i.e., they don't depend on the procedure context. Memory addresses can be in the range $[0, 2^{32})$.

Memory is guaranteed to be initialized to zeros. Thus, when reading from memory address which hasn't been written to previously, zero elements will be returned.

| Instruction     | Stack_input | Stack_output | Notes                                      |
| --------------- | ----------- | ------------ | ------------------------------------------ |
| mem_load <br> - *(1 cycle)*  <br> mem_load.*a* <br> - *(2 cycles)*   | [a, ... ] | [v, ... ] | $v \leftarrow mem[a][0]$ <br> Reads a word (4 elements) from memory at address *a*, and pushes the first element of the word onto the stack. If $a$ is provided via the stack, it is removed from the stack first. <br> Fails if $a \ge 2^{32}$ |
| mem_loadw <br> - *(1 cycle)*  <br> mem_loadw.*a* <br> - *(2 cycles)*  | [a, 0, 0, 0, 0, ... ] | [A, ... ] | $A \leftarrow mem[a]$ <br> Reads a word from memory at address $a$ and overwrites top four stack elements with it. If $a$ is provided via the stack, it is removed from the stack first. <br> Fails if $a \ge 2^{32}$ |
| mem_store <br> - *(2 cycles)*  <br> mem_store.*a*  <br> - *(3-4 cycles)*   | [a, v, ... ] | [ ... ] | $v \rightarrow mem[a][0]$ <br> Pops the top element off the stack and stores it as the first element of the word in memory at address $a$. All other elements of the word are not affected. If $a$ is provided via the stack, it is removed from the stack first. <br> Fails if $a \ge 2^{32}$ |
| mem_storew <br> - *(1 cycle)*  <br> mem_storew.*a* <br> - *(2-3 cycles)*  | [a, A, ... ] | [A, ... ] | $A \rightarrow mem[a]$ <br> Stores the top four elements of the stack in memory at address $a$. If $a$ is provided via the stack, it is removed from the stack first. <br> Fails if $a \ge 2^{32}$ |
| mem_stream <br> - *(2 cycles)* | [S2, S1, S0, a, ...] | [T2, T1, T0, b, ...] | $[T_0, T_1, T_2] \leftarrow permute(S_0, mem[a], mem[a+1])$ <br> $b \leftarrow a + 2$ <br> Loads two words from memory starting at the address $a$, overwrites the top 8 elements of the stack with them, and applies Rescue Prime Optimized permutation to the top 12 elements of the stack. At the end of the operation the address is incremented by $2$. |

The second way to access memory is via procedure locals using the instructions listed below. These instructions are available only in procedure context. The number of locals available to a given procedure must be specified at [procedure declaration](./code_organization.md#procedures) time, and trying to access more locals than was declared will result in a compile-time error. The number of locals per procedure is not limited, but the total number of locals available to all procedures at runtime must be smaller than $2^{32}$.

| Instruction     | Stack_input | Stack_output | Notes                                      |
| --------------- | ----------- | ------------ | ------------------------------------------ |
| loc_load.*i* <br> - *(3-4 cycles)*  | [ ... ] | [v, ... ] | $v \leftarrow local[i][0]$ <br> Reads a word (4 elements) from local memory at index *i*, and pushes the first element of the word onto the stack. |
| loc_loadw.*i*  <br> - *(3-4 cycles)* | [0, 0, 0, 0, ... ] | [A, ... ] | $A \leftarrow local[i]$ <br> Reads a word from local memory at index $i$ and overwrites top four stack elements with it. |
| loc_store.*i* <br> - *(4-5 cycles)*  | [v, ... ] | [ ... ] | $v \rightarrow local[i][0]$ <br> Pops the top element off the stack and stores it as the first element of the word in local memory at index $i$. All other elements of the word are not affected. |
| loc_storew.*i* <br> - *(3-4 cycles)*  | [A, ... ] | [A, ... ] | $A \rightarrow local[i]$ <br> Stores the top four elements of the stack in local memory at index $i$. |

Unlike regular memory, procedure locals are not guaranteed to be initialized to zeros. Thus, when working with locals, one must assume that before a local memory address has been written to, it contains "garbage".

Internally in the VM, procedure locals are stored at memory offset stating at $2^{30}$. Thus, every procedure local has an absolute address in regular memory. The `locaddr.i` instruction is provided specifically to map an index of a procedure's local to an absolute address so that it can be passed to downstream procedures, when needed.
