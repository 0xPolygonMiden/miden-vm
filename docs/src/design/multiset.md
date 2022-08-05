# Multiset checks
A brief introduction on permutation check can be found [here](https://hackmd.io/@arielg/ByFgSDA7D).

This section has borrowed quite heavily from [here](https://github.com/maticnetwork/miden/discussions/238#discussioncomment-2915207). 

Miden VM has few separate dedicated circuits which helps in offloading heavy computation from the VM where things can be done very cheaply while still allowing us to prove correct execution. 

For example, instead of directly executing an expensive operation that could take many cycles, such as a bitwise `AND`, it would be convenient to just look up the value in a table and use whatever value the table gives us. If we could do that, then we could get the result of the bitwise `AND` in a single cycle by just doing a lookup.

To enable this in the bitwise case, we have a special “Bitwise co-processor” which is our equivalent of a specialized circuit that will return the result of bitwise operations in a way that:

1. allows us to enforce constraints and prove the correctness of the result for the given inputs.
2. allows us to do this more efficiently then if we actually had to execute this without a specialized design.

The co-processor takes in the two inputs and the bitwise operation and then provably computes the result so we could just look it up, which is exactly what we need(More information on [bitwise design doc](./chiplets/bitwise.md)).

But now we’re left with a problem - we can prove that the bitwise computation is done correctly, but it’s not connected to our main processor yet, so we still need a way to actually “look up” the value and prove that the value we’re using after our lookup is in fact the correct result from that co-processor for the inputs we specified.

**NOTE** - The stack and the lookup value might not necessarily be sharing the same trace. In fact, the likelihood of both sharing the same/adjacent row is very low. Given STARK constraints are local(constraints can be applied only on two consecutive traces), we can't enforce any transition constraint here.

**Permutation checks are what allow us to link these different processors and prove that this lookup is being executed correctly.**

### Cost of running product columns
It is important to note that depending on the field in which we operate, a running product column may actually require more than one trace columns. This is specifically true for small field.

For example, if we are in a 64-bit field, each running product column would need to be represented by $2$ columns to achieve ~100-bit security, and by $3$ columns to achieve ~128-bit security.

## Virtual tables
TODO: copied from the description of stack overflow table, but needs to be adapted to be generic

To represent this table, we'll use column $p_1$. This column will keep a running product of rows inserted and deleted from the table. Thus, at any step of the computation $p_1$ will contain a product of all rows currently in the table.

To compute a product of rows, we'll first need to reduce each row to a single value. This will be done as follows. Assume the verifier sends a set of random values $\alpha_0$, $\alpha_1$ etc. to the prover after the prover commits to the execution trace of the program. The prover reduces row $i$ in the table to a single value $r_i$ as:

$$
r_i = \alpha_0 + \alpha_1 \cdot t_{0, i} + \alpha_2 \cdot t_{1, i} + \alpha_3 \cdot t_{2, i}
$$

Then, when row $i$ is added to the table, we'll update the value in the $p_1$ column like so:

$$
p_1' = p_1 \cdot r_i
$$

where, $p_1$ denotes the value of column $p_1$ at the current step and $p_1'$ denotes the value of column $p_1$ at the next step.

Analogously, when row $i$ is removed from the table, we'll update the value in column $p_1$ like so:

$$
p_1' = \frac{p_1}{r_i}
$$

The initial value of $p_1$ is set to $1$. Thus, if by the time Miden VM finishes executing a program the table is empty (we added and then removed exactly the same set of rows), $p_1$ will also be equal to $1$.

## Communication buses
TODO