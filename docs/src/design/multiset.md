# Multiset checks

Zero knowledge virtual machines frequently make use of lookup arguments to enable performance optimizations. Miden VM uses multiset checks, and a brief introduction to them can be found [here](https://hackmd.io/@arielg/ByFgSDA7D).

In Miden VM, multiset checks are used for two purposes:

1. To prove the consistency of intermediate values that must persist between different cycles of the trace, without storing the full data in the execution trace (which would require adding more columns to the trace).
2. To prove correct interaction between two independent sections of the execution trace, e.g., between the main trace where the result of some operation is required, but would be expensive to compute, and a specialized component which can perform that operation cheaply.

The first is achieved using [virtual tables](#virtual-tables) of data, where we add a row at some cycle in the trace and remove it at a later cycle when it is needed again. Instead of maintaining the entire table in the execution trace, multiset checks allow us to prove data consistency of this table using one running product column.

The second is done by reducing each operation to a lookup value and then using a [communication bus](#communication-buses), implemented as a running product column, to provably connect the two sections of the trace.

### Running product columns
Although the multiset equality check can be thought of as comparing multiset equality between two vectors $a$ and $b$, in Miden VM it is implemented as a single running product column in the following way:

- The running product column is initialized to a value $x$ at the beginning of the trace. (We typically use $x = 1$.)
- All values of $a$ are multiplied into the running product column.
- All values of $b$ are divided out of the running product column.
- If $a$ and $b$ were multiset equal, then the running product column will equal $x$ at the end of the trace.

Running product columns are computed using a set of random values $\alpha_0$, $\alpha_1, ...$ sent to the prover by the verifier after the prover commits to the execution trace of the program.

#### Length of running product columns

Running product columns are computed by including information from the *current* row of the main execution trace into the *next* row of the running product column. Thus, in order to ensure that the trace is long enough to give the running product column space for its final value, a padding row may be required at the end of the trace of the component upon which the running product column depends.

This is true when the data in the main trace could go all the way to the end of the trace, such as in the case of the range checker.

#### Cost of running product columns
It is important to note that depending on the field in which we operate, a running product column may actually require more than one trace column. This is specifically true for small fields.

Since Miden uses a 64-bit field, each running product column needs to be represented by $2$ columns to achieve ~100-bit security and by $3$ columns to achieve ~128-bit security.

## Virtual tables

Virtual tables can be used to store intermediate data which is computed at one cycle and used at a different cycle. When the data is computed, the row is added to the table, and when it is used later, the row is deleted from the table. Thus, all that needs to be proved is the data consistency between the row that was added and the row that was deleted.

The consistency of a virtual table can be proved with a single trace column $p$, which keeps a running product of rows that were inserted into and deleted from the table. This is done by reducing each row to a single value, multiplying the value into $p$ when the row is inserted, and dividing the value out of $p$ when the row is removed. Thus, at any step of the computation, $p$​ will contain a product of all rows currently in the table.

The initial value of $p$​ is set to 1. Thus, if the table is empty by the time Miden VM finishes executing a program (we added and then removed exactly the same set of rows), the final value of $p$​ will also be equal to 1. The initial and final values are enforced via boundary constraints.

### Computing a virtual table's trace column

To compute a product of rows, we'll first need to reduce each row to a single value. This can be done as follows.

Let $t_0, t_1, t_2, ...$ be columns in the virtual table, and assume the verifier sends a set of random values $\alpha_0$, $\alpha_1, ...$ to the prover after the prover commits to the execution trace of the program.

The prover reduces row $i$ in the table to a single value $r_i$ as:

$$
r_i = \alpha_0 + \alpha_1 \cdot t_{0, i} + \alpha_2 \cdot t_{1, i} + \alpha_3 \cdot t_{2, i} + ...
$$

Then, when row $i$ is added to the table, we'll update the value in the $p$ column like so:

$$
p' = p \cdot r_i
$$

Analogously, when row $i$ is removed from the table, we'll update the value in column $p$ like so:

$$
p' = \frac{p}{r_i}
$$

### Virtual tables in Miden VM

Miden VM currently makes use of 6 virtual tables across 4 components:

- Stack:
    - [Overflow table](./stack/main.md#overflow-table)
- Decoder:
    - [Block stack table](./decoder/main.md#block-stack-table)
    - [Block hash table](./decoder/main.md#block-hash-table)
    - [Op group table](./decoder/main.md#op-group-table)
- Range checker:
    - [8-bit range checks table](./range.md#8-bit-range-checks-table)
- Hash chiplet:
    - [Sibling table](./chiplets/hasher.md#sibling-table-constraints)

## Communication buses

One strategy for improving the efficiency of a zero knowledge virtual machine is to use specialized components for complex operations and have the main circuit “offload” those operations to the corresponding components by specifying inputs and outputs and allowing the proof of execution to be done by the dedicated component instead of by the main circuit.

These specialized components are designed to prove the internal correctness of the execution of the operations they support. However, in isolation they cannot make any guarantees about the source of the input data or the destination of the output data.

In order to prove that the inputs and outputs specified by the main circuit match the inputs and outputs provably executed in the specialized component, some kind of provable communication bus is needed.

This bus is typically implemented as some kind of lookup argument, and in Miden VM in particular we use multiset checks.

### Implementation

A `bus` can be implemented as a single trace column $b$ where a request can be sent to a specific component and a corresponding response will be sent back by that component.

The values in this column contain a running product of the communication with the component as follows:

- Each request is “sent” by computing a lookup value from some information that's specific to the specialized component, the operation inputs, and the operation outputs, and then dividing it out of the running product column $b$.
- Each chiplet response is “sent” by computing the same lookup value from the component-specific information, inputs, and outputs, and then multiplying it into the running product column $b$.

Thus, if the requests and responses match, and the bus column $b$ is initialized to $1$, then $b$ will start and end with the value $1$. This condition is enforced by boundary constraints on column $b$.

Note that the order of the requests and responses does not matter, as long as they are all included in $b$. In fact, requests and responses for the same operation will generally occur at different cycles. Additionally, there could be multiple requests sent in the same cycle, and there could also be a response provided at the same cycle that a request is received.

### Communication bus constraints

These constraints can be expressed in a general way with the 2 following requirements:

- The lookup value must be computed using random values $\alpha_0, \alpha_1$, etc. that are provided by the verifier after the prover has committed to the main execution trace.
- The lookup value must include all uniquely identifying information for the component/operation and its inputs and outputs.

Given an example operation $op_{ex}$ with inputs $i_0, ..., i_n$ and outputs $o_0, ..., o_m$, the lookup value can be computed as follows:

$$lookup = \alpha_0 + \alpha_1 \cdot op_{ex} + \alpha_2 \cdot i_0 + ... + \alpha_{n+2} \cdot i_n + \alpha_{n+3} \cdot o_0 + ... + \alpha_{n + 2 + m} \cdot o_m$$

The constraint for sending this to the bus as a request would be:

$$b' \cdot lookup = b$$

The constraint for sending this to the bus as a response would be:

$$b' = b \cdot lookup$$

However, these constraints must be combined, since it's possible that requests and responses both occur during the same cycle.

To combine them, let $u_{lookup}$ be the request value and let $v_{lookup}$ be the response value. These values are both computed the same way as shown above, but the data sources are different, since the input/output values used to compute $u_{lookup}$ come from the trace of the component that's "offloading" the computation, while the input/output values used to compute $v_{lookup}$ come from the trace of the specialized component.

The final constraint can be expressed as:

$$b' \cdot u_{lookup} = b \cdot v_{lookup}$$

### Communication buses in Miden VM

In Miden VM, the specialized components are implemented as dedicated segments of the execution trace, which include the range checker and the 3 chiplets in the Chiplets module (the hash chiplet, bitwise chiplet, and memory chiplet).

Miden VM currently uses 2 buses to communicate with these components:

- The chiplets bus [$b_{chip}$](./chiplets/main.md#chiplet-bus), which communicates with all of the chiplets (Hash, Bitwise, and Memory).
- The range checker bus [$b_{range}$](./range.md#communication-bus).
