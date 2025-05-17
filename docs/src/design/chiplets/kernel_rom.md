# Kernel ROM chiplet

The kernel ROM enables executing predefined kernel procedures. These procedures are always executed in the root context
and can only be accessed by a `SYSCALL` operation. The chiplet tracks and enforces correctness of all kernel procedure
calls as well as maintaining a list of all the procedures defined for the kernel, whether they are executed or not. More
background about Miden VM execution contexts can be found [here](../../user_docs/assembly/execution_contexts.md).

## Kernel ROM trace

The kernel ROM table consists of 5 columns.

> TODO: Update diagram
> - remove `idx`
> - duplicate each hash once and set `s_first` = 1 in that row

![kernel_rom_execution_trace](../../assets/design/chiplets/kernel_rom/kernel_rom_execution_trace.png)

The meaning of columns in the above is as follows:

- Column $s_{first}$ specifies that the start of a block representing a unique kernel procedure hash.
- $r_0, ..., r_3$ are contain the roots of the kernel functions. The values in these columns can change only
  when $s_{first}$ is set to 1 in the next row. Otherwise, the values in the $r$ columns remain the same.

## Constraints

> Note: the following assumes the ACE chiplet is included in the previous slot, whose documentation will be included
> in a subsequent PR.

The following constraints are required to enforce the correctness of the kernel ROM trace.
_Note: Unless otherwise stated, these constraints should also be multiplied by chiplets module's selector
flag $chip\_s_4$ for the kernel ROM chiplet, as is true for all constraints in this chiplet._

The $s_{first}$ column must be binary.

> $$
s_{first}^2 - s_{first} = 0 \text{ | degree} = 2
$$

$s_{first}$ must be set in the first row of the trace, indicating the start of a new hash which must be included in the set of kernel procedure roots provided by the verifier.
This constraint is enforced in the last row of the previous trace, using selector columns from the [chiplets](main.md)
module.
The virtual $chip\_s_3$ flag is active in all rows of the previous chiplet.
In the last row of that chiplet, the selector $s_3$ transitions from 0 to 1.

> $$
chip\_s_3 \cdot s_3' \cdot (1 - s_{first}') = 0 \text{ | degree} = \deg(chip\_s_3) + 2
$$

_Note that this selector need not be multiplied by the kernel ROM chiplet flag $chip\_s_4$, since it is only active when the previous chiplet is active._

The hash of the procedure repeats in the next row whenever the next row is not the first of a new procedure, i.e., when $s_{first}' = 0$.
To ensure the constraint is disabled in the last row, we multiply it by the chiplet selector $s_4$ for the next row,
since it transitions from 0 to 1 at this point, indicating the start of

> $$
s_4' \cdot (1 - s_{first}') \cdot (r_i' - r_i) = 0 \text{ | degree} = 3
$$

### Kernel procedure table constraints

This kernel procedure table keeps track of all *unique* kernel function roots. The values in this table will be updated
only when the value in the $s_{first}$ selector is set.

The chiplet communicates with the chiplet bus via the $b_{chip}$ columns.
In the first row of each trace segment containing the same hash, it responds to a request with message $u_{init}$,
ensuring equality between the set of hashes contained in the trace and the list of kernel procedure roots provided via
public inputs.
In all other rows, the chiplet responds messages $u_{bus}$ requested during program block hashing of the [
`SYSCALL` operation](../decoder/constraints.md#block-hash-computation-constraints).

The variables $u_{init}$ and $u_{bus}$ represent reduced bus messages containing a kernel procedure hash.
Denoting the random values received from the verifier as $\alpha_0, \alpha_1$, etc., this can be defined as

$$
\begin{aligned}
u_{init} &= \alpha_0 + \sum_{i=0}^3 (\alpha_{i + 1} \cdot r_i) \\
u_{bus} &= \alpha_0 + \alpha_1 \cdot op_{krom} + \sum_{i=0}^3 (\alpha_{i + 2} \cdot r_i) \\
\end{aligned}
$$

Here, $op_{krom}$ is the unique [operation label](./main.md#operation-labels) of the kernel procedure call operation.

_Note: the above might be unsafe since the first element of a hash may collide with another bus message op code._

To provide accessed kernel procedures to the chiplets bus, we must send the kernel procedure to the bus every time it is
called, which is indicated by the $s_{first}$ column.
We also ensure that each new hash in the table corresponds to one of the kernel procedure roots provided via public inputs.
These responses are mutually exclusive, allowing us to group them in a single constraint.

> $$
b'_{chip} = b_{chip} \cdot (s_{first} \cdot u_{init} + (1 - s_{first}) \cdot u_{bus}) \text{ | degree} = 3
$$

We also need to impose boundary constraints to ensure that the set of unique hashes in the trace matches the set of kernel procedure roots provided by the verifier.
This is achieved by enforcing that the running product column for the bus is initialized with all $u_{init}$ hashes provided by the verifier.
This ensures that the verifier only needs to know which kernel was used, but doesn't need to know which functions were invoked within the kernel.
Moreover, since each new hash in the trace initiates a response to the initial public input requests, the trace cannot contain any other hash, ensuring both sets of hashes are equal.

This constraint is described as part of
the [chiplets bus constraints](../chiplets/main.md#chiplets-bus-constraints).

