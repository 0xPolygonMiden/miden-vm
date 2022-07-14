# Unique Identifier

This sections describes how we are assigning a unique identifier to each co-processor operations which are used in permutation check. A brief introduction about permutation check can be found [here](https://hackmd.io/@arielg/ByFgSDA7D).

## Permutation check in processor

It has been explained in detail by @grjte [here](https://github.com/maticnetwork/miden/discussions/238#discussioncomment-2915207). 

Miden VM has few separate dedicated circuits which helps in offloading heavy computation from the VM where things can be done very cheaply while still allowing us to prove correct execution. 

For example, instead of directly executing an expensive operation that could take many cycles, such as a bitwise `AND`, it would be convenient to just look up the value in a table and use whatever value the table gives us. If we could do that, then we could get the result of the bitwise `AND` in a single cycle by just doing a lookup.

To enable this in the bitwise case, we have a special “Bitwise co-processor” which is our equivalent of a specialized circuit that will return the result of bitwise operations in a way that:

1. allows us to enforce constraints and prove the correctness of the result for the given inputs.
2. allows us to do this more efficiently then if we actually had to execute this without a specialized design.

The co-processor takes in the two inputs and the bitwise operation and then provably computes the result so we could just look it up, which is exactly what we need(More information on [bitwise design doc](../aux_table/bitwise.md)).

But now we’re left with a problem - we can prove that the bitwise computation is done correctly, but it’s not connected to our main processor yet, so we still need a way to actually “look up” the value and prove that the value we’re using after our lookup is in fact the correct result from that co-processor for the inputs we specified.

**NOTE** - The stack and the lookup value might not be at the same trace. There a very high likelihood that they are at a different trace and given we STARK constraint are all local(constraints can be applied only on two consecutive traces) we cant apply any constraint here. 

**Permutation checks are what allow us to link these different processors and prove that this lookup is being executed correctly.**

Miden VM has a running product column $p_0$ which is used to tie the co-processor with the main VM's stack. When receiving inputs from or returning results to the stack, the procesor multiplies $p_0$ by their respective values(unique identifier is a component of it). On the other side, when sending inputs to these co-processor or receiving results from the co-processor, the stack divides $p_0$ by their values. We use boundary constraints to ensure that the running product column started and ended with a value of 1. If that is the case, then all of our lookups must have matched all of the computations that were executed in the co-processor (all of which were provably correct).

We have created a separate unique identifier of an operation which is a component of this permutation check to further ensure that the values stack is looking up from the "lookup" table is indeed coming from the intended co-processor for that operation and not from somewhere else. 

## Identifiers

The identifier are made up using selector and internal selector(if they have it) flags values of an operation. The internal selectors flag are joined with the selector flag values first followed by a binary aggregation of this combined value. When $1$ is added to it, we get the unique value of the operation. 


| Operation | Selector flag | Internal Selector Flag | Combined flag | Unique Identifier | 
| --------- | :-----------: | :--------------------: | ------------- | ----------------- |
| `HASHER_LINER_HASH`    | $\{0\}$       | $\{1, 0, 0\}$ | $\{0, 1, 0, 0\}$ | 3  |
| `HASHER_MP_VERIFY`     | $\{0\}$       | $\{1, 0, 1\}$ | $\{0, 1, 0, 1\}$ | 11 |
| `HASHER_MR_UPDATE_OLD` | $\{0\}$       | $\{1, 1, 0\}$ | $\{0, 1, 1, 0\}$ | 7  |
| `HASHER_MR_UPDATE_NEW` | $\{0\}$       | $\{1, 1, 1\}$ | $\{0, 1, 1, 1\}$ | 15 |
| `HASHER_RETURN_HASH`   | $\{0\}$       | $\{0, 0, 0\}$ | $\{0, 0, 0, 0\}$ | 1  |
| `HASHER_RETURN_STATE`  | $\{0\}$       | $\{0, 0, 1\}$ | $\{0, 0, 0, 1\}$ | 9  |
| `BITWISE_AND`          | $\{1, 0\}$    | $\{0, 0\}$    | $\{1, 0, 0, 0\}$ | 2  |
| `BITWISE_OR`           | $\{1, 0\}$    | $\{0, 1\}$    | $\{1, 0, 0, 1\}$ | 10 |
| `BITWISE_XOR`          | $\{1, 0\}$    | $\{1, 0\}$    | $\{1, 0, 1, 0\}$ | 6  |
| `MEMORY`               | $\{1, 1, 1\}$ | NA            | $\{1, 1, 1\}$    | 8  |

