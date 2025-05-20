# Kernel ROM chiplet

The kernel ROM enables executing predefined kernel procedures.
These procedures are always executed in the root context and can only be accessed by a `SYSCALL` operation.
The chiplet tracks and enforces correctness of all kernel procedure calls as well as maintaining a list of all the procedures defined for the kernel, whether they are executed or not.
More background about Miden VM execution contexts can be found [here](../../user_docs/assembly/execution_contexts.md).

## Kernel ROM trace

The kernel ROM table consists of five columns.

> TODO: Update diagram
> - remove `idx`
> - duplicate each digest once and set `s_first` = 1 in that row

![kernel_rom_execution_trace](../../assets/design/chiplets/kernel_rom/kernel_rom_execution_trace.png)

The meaning of columns in the above is as follows:

- Column $s_{first}$ specifies the start of a block of rows with identical kernel procedure digests.
- $r_0, ..., r_3$ contain the digests of the kernel functions. The values in these columns can change only when $s_{first}$ is set to 1 in the next row. Otherwise, the values in the $r$ columns remain the same.

## Constraints

> Note: the following assumes the ACE chiplet is included in the previous slot, whose documentation will be included
> in a subsequent PR.

The following constraints are required to enforce the correctness of the kernel ROM trace.

_Note: Unless otherwise stated, these constraints should also be multiplied by chiplets module's selector flag $chip\_s_4$ for the kernel ROM chiplet._

The $s_{first}$ column is a selector indicating the start of a new digest included in the kernel ROM chiplet trace.
In this row, the chiplet responds to a bus request made by the verifier to ensure consistency with the set of kernel procedure digests given as public inputs.

As $s_{first}$ is a selector, it must be binary.

> $$
s_{first}^2 - s_{first} = 0 \text{ | degree} = 2
$$


The flag $s_{first}$ must be set to be 1 in the first row of the kernel ROM chiplet.
Otherwise, the digest in this row would not be matched with one of the input procedure roots.
This constraint is enforced in the last row of the previous trace, using selector columns from the [chiplets](main.md) module.
The virtual $chip\_s_3$ flag is active in all rows of the previous chiplet,
and in the last row of that chiplet, the selector $s_3$ transitions from 0 to 1.

> $$
chip\_s_3 \cdot s_3' \cdot (1 - s_{first}') = 0 \text{ | degree} = \deg(chip\_s_3) + 2
$$

_Note that this selector need not be multiplied by the kernel ROM chiplet flag $chip\_s_4$, since it is only active when the previous chiplet is active._

The contiguity of the digests in a block is ensured by enforcing equality between digests across two consecutive rows, whenever the next row is not the start of a new block.
That is, when $s_{first}' = 0$, it must hold that $r_i = r_i'$.
We disable this constraint in the last row of the kernel ROM chiplet trace by using the kernel ROM chiplet selector $s_4'$, since the latter transitions from 0 to 1 when the next chiplet starts.

> $$
(1 - s_4') \cdot (1 - s_{first}') \cdot (r_i' - r_i) = 0 \text{ | degree} = 3
$$

_**Note**: we could technically remove the selector $(1-s_4')$ since $s_4$ and $s_{first}$ correspond to the same column. We include it here for completeness though._

### Chiplets bus constraints

The kernel ROM chiplet must ensure that all kernel procedure digests requested by the decoder correspond to one of the digests provided by the verifier through public inputs.
This is achieved by making use of the chiplet bus $b_{bus}$, responding to requests made by the decoder and by the verifier through public inputs.

In the first row of each new block of hashes in the kernel ROM chiplet trace (i.e., when $s_{first} = 1$), the chiplet responds to a message $v_{init}$ requested by the verifier.
Since these initialization messages must match, the set of digest across all blocks must be equal to the set of procedure roots provided by the verifier (though not necessarily in the same order).

Whenever a digest is requested by the decoder during program block hashing of the [`SYSCALL` operation](../decoder/constraints.md#block-hash-computation-constraints), a new row is added to the trace after the first row which initialized that digest.
The chiplet responds to the request with a message $v_{call}$.

In other words, the selector $s_{first}$ indicates whether the chiplet should respond to the decoder or the verifier initialization requests.
If a digest is requested $n$ times by the decoder, the same digest appears in a single block of length $n+1$.

The variables $v_{init}$ and $v_{call}$ representing the bus messages contain reduced bus messages containing a kernel procedure digest.
Denoting the random values received from the verifier as $\alpha_0, \alpha_1$, etc., this can be defined as

$$
\begin{aligned}
\tilde{r} &= \sum_{i=0}^3 (\alpha_{i + 2} \cdot r_i) \\
v_{init} &= \alpha_0 + \alpha_1 \cdot \textsf{KERNEL_PROC_INIT_LABEL} + \tilde{r} \\
v_{call} &= \alpha_0 + \alpha_1 \cdot \textsf{KERNEL_PROC_CALL_LABEL} + \tilde{r}
\end{aligned}
$$

Here, $\textsf{KERNEL_PROC_INIT}$ and $\textsf{KERNEL_PROC_CALL}$ are the unique [operation labels](./main.md#operation-labels) for the kernel ROM bus message.

Each row of the kernel ROM chiplet trace responds to either a procedure digest initialization or decoder call request.
Since the $s_{first}$ column defines which type of response is sent to the bus, it is used to combine both requests into a single constraint given by

> $$
b'_{chip} = b_{chip} \cdot (s_{first} \cdot v_{init} + (1 - s_{first}) \cdot v_{call}) \text{ | degree} = 3.
$$

The above simplifies to

- $s_{first} = 1$: $b'_{chip} = b_{chip} \cdot v_{init}$, when responding to a $\textsf{KERNEL_PROC_INIT}$ request.
- $s_{first} = 0$: $b'_{chip} = b_{chip} \cdot v_{call}$, when responding to a $\textsf{KERNEL_PROC_CALL}$ request.

The kernel procedure digests initialization requests are implemented by imposing a boundary constraint in the first row of the $b_{chip}$ column.
This is described in the [chiplets bus constraints](../chiplets/main.md#chiplets-bus-constraints).

By using the bus to initialize the kernel ROM procedure digest in this way, the verifier only learns which procedures can be invoked but doesn't learn how often they were called.

The full set of constraints applied to the $b_{chip}$ are described as part of the [chiplets bus constraints](../chiplets/main.md#chiplets-bus-constraints).

