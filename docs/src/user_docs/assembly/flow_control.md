## Flow control
Miden assembly provides high-level constructs to facilitate flow control. These constructs are:

- *if-else* expressions for conditional execution.
- *repeat* expressions for bounded counter-controlled loops.
- *while* expressions for unbounded condition-controlled loops.
- *calls* to kernel functions (`syscall`) and user-defined functions (`call`)

### Conditional execution
Conditional execution in Miden VM can be accomplished with *if-else* statements. These statements look like so:
```
if.true
    <instructions>
else
    <instructions>
end
```
where `instructions` can be a sequence of any instructions, including nested control structures; the `else` clause is optional. The above does the following:

1. Pops the top item from the stack.
2. If the value of the item is $1$, instructions in the `if.true` branch are executed.
3. If the value of the item is $0$, instructions in the `else` branch are executed.
4. If the value is not binary, the execution fails.

A note on performance: using *if-else* statements incurs a small, but non-negligible overhead. Thus, for simple conditional statements, it may be more efficient to compute the result of both branches, and then select the result using [conditional drop](./stack_manipulation.md#conditional-manipulation) instructions.

### Counter-controlled loops
Executing a sequence of instructions a predefined number of times can be accomplished with *repeat* statements. These statements look like so:
```
repeat.<count>
    <instructions>
end
```
where:

* `instructions` can be a sequence of any instructions, including nested control structures.
* `count` is the number of times the `instructions` sequence should be repeated (e.g. `repeat.10`). `count` must be an integer greater than $0$.

### Condition-controlled loops
Executing a sequence of instructions zero or more times based on some condition can be accomplished with *while loop* expressions. These expressions look like so:
```
while.true
    <instructions>
end
```
where `instructions` can be a sequence of any instructions, including nested control structures. The above does the following:

1. Pops the top item from the stack.
2. If the value of the item is $1$, `instructions` in the loop body are executed.
    a. After the body is executed, the stack is popped again, and if the popped value is $1$, the body is executed again.
    b. If the popped value is $0$, the loop is exited.
    c. If the popped value is not binary, the execution fails.
3. If the value of the item is $0$, execution of loop body is skipped.
4. If the value is not binary, the execution fails.

Example:

```
# push the boolean true to the stack
push.1

# pop the top element of the stack and loop while it is true
while.true
    # push the boolean false to the stack, finishing the loop for the next iteration
    push.0
end
```

### Function calls

Miden VM has 2 separate executions contexts for functions: kernel space and user space. Each context imposes restrictions on the functions that can be executed there and has its own isolated memory context. Every program starts execution in kernel space (memory context = 0).


#### `syscall` instruction

The `syscall` instruction can be used to execute kernel functions and to make calls into kernel space from user space.

For example, the following could be defined as a kernel and provided to the Assembler (using the `with_kernel` method).

```
export.foo
    add
end
```

The kernel function can then be called from program code:

```
begin
    syscall.foo
end
```

When the VM executes `syscall` it does the following:

1. Make sure that specified function actually belongs to the set of available kernel functions.
2. Set memory context to 0 (the context ID for kernel space)
3. Put the hash of the currently executing function onto the stack.
4. Execute the actual code of the function.

The kernel function has access to the hash of the caller, so it infers the privilege level of the caller. It is not possible to execute a `call` or another `syscall` while executing a `syscall`.

#### `call` instruction

The `call` instruction can be used to execute programs in the user space.

When calling a function in user space, the function will be executed in its own memory context and will not have access to the root context.

```
proc.foo
    # store 3 in an isolated memory context at address 0
    push.3
    mem_store.0
end

begin
    # store 7 in the root memory context at address 0
    push.7
    mem_store.0
    # foo is executed in a different memory context
    call.foo
    # thus the value saved into memory[0] before calling foo will still be there.
    mem_load.0
    eq.7
    assert
end
```

When the VM executes `call` it does the following:

1. Set the memory context to some new unique value so that the function code doesn't have access to the kernel memory.
2. Execute the actual code of the function.