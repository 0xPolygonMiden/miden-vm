# Hash Processor

This note assumes some familiarity with [permutation checks](https://hackmd.io/@arielg/ByFgSDA7D).

Miden VM "offloads" all hash-related computations to a separate _hash processor_. This processor supports executing [Rescue Prime](https://eprint.iacr.org/2020/1143) hash function (or rather a [specific instantiation](https://docs.rs/winter-crypto/0.3.2/winter_crypto/hashers/struct.Rp64_256.html) of it) in the following settings:

- A single permutation of Rescue Prime.
- A simple 2-to-1 hash.
- A linear hash of $n$ field elements.
- Merkle path verification.
- Merkle root update.

The processor can be thought of as having a small instruction set of $11$ instructions. These instructions are listed below, and examples of how these instructions are used by the processor are described in the following sections.

| Instruction | Description                                                                                                                                                                                                                                                                                                        |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `RPR`       | Executes a single round of Rescue Prime. All cycles which are not one less than a multiple of $8$ execute this instruction. That is, the processor executes this instruction on cycles $0, 1, 2, 3, 4, 5, 6$, but not $7$, and then again, $8, 9, 10, 11, 12, 13, 14$, but not $15$ etc.                           |
| `BP`        | Initiates computation of a single permutation, a 2-to-1 hash, or a linear hash of many elements. This instruction can be executed only on cycles which are multiples of $8$, and it can also be executed concurrently with `RPR` instruction.                                                                      |
| `MP`        | Initiates Merkle path verification computation. This instruction can be executed only on cycles which are multiples of $8$, and it can also be executed concurrently with `RPR` instruction.                                                                                                                       |
| `MV`        | Initiates Merkle path verification for the "old" node value during Merkle root update computaiton. This instruction can be executed only on cycles which are multiples of $8$, and it can also be executed concurrently with `RPR` instruction.                                                                    |
| `MU`        | Initiates Merkle path verification for the "new" node value during Merkle root update computaiton. This instruction can be executed only on cycles which are multiples of $8$, and it can also be executed concurrently with `RPR` instruction.                                                                    |
| `HOUT`      | Returns the result of the currently running computation. This instruction can be executed only on cycles which are one less than a multiple of $8$ (e.g. $7$, $15$ etc.).                                                                                                                                          |
| `SOUT`      | Returns the whole hasher state. This instruction can be executed only on cycles which are one less than a multiple of $8$, and only if the computation was started using `BP` instruction.                                                                                                                         |
| `ABP`       | Absorbs a new set of elements into the hasher state when computing a linear hash of many elements. This instruction can be executed only on cycles which are one less than a multiple of $8$, and only if the computation was started using `BP` instruction.                                                      |
| `MPA`       | Absorbs the next Merkle path node into the hasher state during Merkle path verification computation. This instruction can be executed only on cycles which are one less than a multiple of $8$, and only if the computation was started using `MP` instruction.                                                    |
| `MVA`       | Absorbs the next Merkle path node into the hasher state during Merkle path verification for the "old" node value during Merkle root update computation. This instruction can be executed only on cycles which are one less than a multiple of $8$, and only if the computation was started using `MV` instruction. |
| `MUA`       | Absorbs the next Merkle path node into the hasher state during Merkle path verification for the "new" node value during Merkle root update computation. This instruction can be executed only on cycles which are one less than a multiple of $8$, and only if the computation was started using `Mu` instruction. |

## Processor trace

Execution trace table of the processor consists of $17$ trace columns and $3$ periodic columns. The structure of the table is such that a single permutation of the hash function can be computed using $8$ table rows. The layout of the table is illustrated below.

![](https://i.imgur.com/g2oeXQ9.png)

The meaning of the columns is as follows:

- Three periodic columns $k_0$, $k_1$, and $k_2$ are used to help select the instruction executed at a given row. All of these columns contain patterns which repeat every $8$ rows. For $k_0$ the pattern is $7$ zeros followed by $1$ one, helping us identify the last row in the cycle. For $k_1$ the pattern is $6$ zeros, $1$ one, and $1$ zero, which can be used to identify the second-to-last row in a cycle. For $k_2$ the pattern is $1$ one followed by $7$ zeros, which can identify the first row in the cycle.
- Three selector columns $s_0$, $s_1$, and $s_2$. These columns can contain only binary values (ones or zeros), and they are also used to help select the instruction to execute at a given row.
- One row address column $r$. This column starts out at $0$ and gets incremented by $1$ with every row.
- Twelve hasher state columns $h_0, ..., h_{11}$. These columns are used to hold the hasher state for each round of Rescue Prime permutation. The state is laid out as follows:
  - The first four columns ($h_0, ..., h_3$) are reserved for capacity elements of the state. When the state is initialized for hash computations, $h_0$ should be set to the number of elements to be hashed. All other capacity elements should be set to $0$'s.
  - The next eight columns ($h_4, ..., h_{11}$) are reserved for the rate elements of the state. These are used to absorb the values to be hashed. Once the permutation is complete, hash output is located in the first four rate columns ($h_4, ..., h_7$).
- One index column $i$. This column is used to help with Merkle path verification and Merkle root update computations.

In addition to the columns described above, the table relies on two running product columns which are used to facilitate permutation checks. These columns are:

- $p_0$ - which is used to tie the processor table with with the main VM's stack. That is, inputs consumed by the processor and outputs produced by the processor are added to $p_0$, while the main VM stack removes them from $p_0$. Thus, if the sets of inputs and outputs between the main VM stack and hash processor are the same, value of $p_0$ should be equal to $1$ at the start and the end of the execution trace.
- $p_1$ - which is used to as a _virtual_ helper table for Merkle root update computations.

## Instruction flags

As mentioned above, processor instructions are encoded using a combination of periodic and selector columns. These columns can be used to compute a binary flag for each instruction. Thus, when a flag for a given instruction is set to $1$, the processor executes this instruction. Formulas for computing instruction flags are listed below.

| Flag       | Value                                                 | Notes                                                                                             |
| ---------- | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| $f_{rpr}$  | $1 - k_0$                                             | Set to $1$ on the first $7$ steps of every $8$-step cycle.                                        |
| $f_{bp}$   | $k_2 \cdot s_0 \cdot (1 - s_1) \cdot (1 - s_2)$       | Set to $1$ when selector flags are $(1, 0, 0)$ on rows which are multiples of $8$.                |
| $f_{mp}$   | $k_2 \cdot s_0 \cdot (1 - s_1) \cdot s_2$             | Set to $1$ when selector flags are $(1, 0, 1)$ on rows which are multiples of $8$.                |
| $f_{mv}$   | $k_2 \cdot s_0 \cdot s_1 \cdot (1 - s_2)$             | Set to $1$ when selector flags are $(1, 1, 0)$ on rows which are multiples of $8$.                |
| $f_{mu}$   | $k_2 \cdot s_0 \cdot s_1 \cdot s_2$                   | Set to $1$ when selector flags are $(1, 1, 1)$ on rows which are multiples of $8$.                |
| $f_{hout}$ | $k_0 \cdot (1 - s_0) \cdot (1 - s_1) \cdot (1 - s_2)$ | Set to $1$ when selector flags are $(0, 0, 0)$ on rows which are $1$ less than a multiple of $8$. |
| $f_{sout}$ | $k_0 \cdot (1 - s_0) \cdot (1 - s_1) \cdot s_2$       | Set to $1$ when selector flags are $(0, 0, 1)$ on rows which are $1$ less than a multiple of $8$. |
| $f_{abp}$  | $k_0 \cdot s_0 \cdot (1 - s_1) \cdot (1 - s_2)$       | Set to $1$ when selector flags are $(1, 0, 0)$ on rows which are $1$ less than a multiple of $8$. |
| $f_{mpa}$  | $k_0 \cdot s_0 \cdot (1 - s_1) \cdot s_2$             | Set to $1$ when selector flags are $(1, 0, 1)$ on rows which are $1$ less than a multiple of $8$. |
| $f_{mva}$  | $k_0 \cdot s_0 \cdot s_1 \cdot (1 - s_2)$             | Set to $1$ when selector flags are $(1, 1, 0)$ on rows which are $1$ less than a multiple of $8$. |
| $f_{mua}$  | $k_0 \cdot s_0 \cdot s_1 \cdot s_2$                   | Set to $1$ when selector flags are $(1, 1, 1)$ on rows which are $1$ less than a multiple of $8$. |

A few additional notes about flag values:

- With the exception of $f_{rpr}$, all flags are mutually exclusive. That is, if one flag is set to $1$, all other flats are set to $0$.
- With the exception of $f_{rpr}$, computing flag values involves $3$ multiplications, and thus the degree of these flags is $4$.
- We can also define a flag $f_{out} = k_0 \cdot (1 - s_0) \cdot (1 - s_1)$. This flag will be set to $1$ when either $f_{hout}=1$ or $f_{sout}=1$ in the current row.
- We can define a flag $f_{out}' = k_1 \cdot (1 - s_0') \cdot (1 - s_1')$. This flag will be set to $1$ when either $f_{hout}=1$ or $f_{sout}=1$ in the next row.

We also impose the following restrictions on how values in selector columns can be updated:

- Values in columns $s_1$ and $s_2$ must be copied over from one row to the next, unless $f_{out} = 1$ or $f_{out}' = 1$ indicating the `hout` or `sout` flag is set for the current or the next row.
- Value in $s_0$ must be set to $1$ if $f_{out}=1$ for the previous row, and to $0$ if any of the flags $f_{abp}$, $f_{mpa}$, $f_{mva}$, or $f_{mua}$ are set to $1$ for the previous row.

The above rules ensure that we must finish one computation before starting another, and we can't change the type of the computation before the computation is finished.

## Computation examples

### Single permutation

Computing a single permutation of Rescue Prime hash function involves the following steps:

1. Initialize hasher state with $12$ field elements.
2. Apply Rescue Prime permutation.
3. Return the entire hasher state as output.

The processor accomplishes the above by executing the following instructions:

```
[BP, RPR]                // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR  // execute 6 more Rescue rounds
SOUT                     // return the entire state as output
```

Execution trace for this computation would look as illustrated below.

![](https://i.imgur.com/RqnZvwH.png)

In the above $\{a_0, ..., a_{11}\}$ is the input state of the hasher, and $\{b_0, ..., b_{11}\}$ is the output state of the hasher.

### Simple 2-to-1 hash

Computing a 2-to-1 hash involves the following steps:

1. Initialize hasher state with $8$ field elements, setting the first capacity element to $8$, and the remaining capacity elements to $0$
2. Apply Rescue Prime permutation.
3. Return elements ${4, ..., 7}$ of the hasher state as output.

The processor accomplishes the above by executing the following instructions:

```
[BP, RPR]                // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR  // execute 6 more Rescue rounds
HOUT                     // return elements 4, 5, 6, 7 of the state as output
```

Execution trace for this computation would look as illustrated below.

![](https://i.imgur.com/nECz9UF.png)

In the above, we compute the following:

$$
\{c_0, c_1, c_2, c_3\} \leftarrow hash(\{a_0, a_1, a_2, a_3\}, \{b_0, b_1, b_2, b_3\})
$$

### Linear hash of n elements

Computing a linear hash of $n$ elements consists of the following steps:

1. Initialize hasher state with the first $8$ elements, setting the first capacity register to $n$, and the remaining capacity elements to $0$.
2. Apply Rescue Prime permutation.
3. Absorb the next set of elements into the state (up to $8$ elements), while keeping capacity elements unchanged.
4. Repeat steps 2 and 3 until all $n$ elements have been absorbed.
5. Return elements ${4, ..., 7}$ of the hasher state as output.

The processor accomplishes the above by executing the following instructions (for hashing $16$ elements):

```
[BP, RPR]                   // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR     // execute 6 more Rescue rounds
ABP                         // absorb the next set of elements into the state
RPR RPR RPR RPR RPR RPR RPR // execute 7 Rescue rounds
HOUT                        // return elements 4, 5, 6, 7 of the state as output
```

Execution trace for this computation would look as illustrated below.

![](https://i.imgur.com/JEFSHJg.png)

In the above, the value absorbed into hasher state between rows $7$ and $8$ is the delta between values $t_i$ and $s_i$. Thus, if we define $b_i = t_i - s_i$ for $i \in \{0, ..., 7\}$, the above computes the following:

$$
\{r_0, r_1, r_2, r_3\} \leftarrow hash(a_0, ..., a_7, b_0, ..., b_7)
$$

### Verify Merkle path

Verifying a Merkle path involves the following steps:

1. Initialize hasher state with the leaf and the first node of the path, setting the first capacity element to $8$, and the remaining capacity elements to $0$s.
   a. Also, initialize the index register to the leaf's index value.
2. Apply Rescue Prime permutation.
   a. Make sure the index value doesn't change during this step.
3. Copy the result of the hash to the next row, and absorb the next node of the Merkle path into the hasher state.
   a. Remove a single bit from the index, and use it to determine how to place the copied result and absorbed node in the state.
4. Repeat steps 2 and 3 until all nodes of the Merkle path have been absorbed.
5. Return elements ${4, ..., 7}$ of the hasher state as output.
   a. Also, make sure the index value has been reduced to $0$.

The processor accomplishes the above by executing the following instructions (for Merkle tree of depth $3$):

```
[MP, RPR]                   // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR     // execute 6 more Rescue rounds
MPA                         // copy result & absorb the next node into the state
RPR RPR RPR RPR RPR RPR RPR // execute 7 Rescue rounds
HOUT                        // return elements 4, 5, 6, 7 of the state as output
```

Suppose we have a Merkle tree as illustrated below. This Merkle tree has $4$ leaves, each of which consists of $4$ field elements. For example, leaf $a$ consists of elements $a_0, a_1, a_2, a_3$, leaf be consists of elements $b_0, b_1, b_2, b_3$ etc.

![](https://hackmd.io/_uploads/Hk4txKNAY.png)

If we wanted to verify that leaf $d$ is in fact in the tree, we'd need to compute the following hashes:

$$
r \leftarrow hash(e, hash(c, d))
$$

And if $r = g$, we can be convinced that $d$ is in fact in the tree at position $3$. Execution trace for this computation would look as illustrated below.

![](https://i.imgur.com/uZdqicd.png)

In the above, the prover provides values for nodes $c$ and $e$ non-deterministically.

### Update Merkle root

Updating a node in a Merkle tree (which also updates the root of the tree) can be simulated by verifying two Merkle paths: the path that starts with the old leaf and the path that starts with the new leaf.

Suppose we have the same Merkle tree as in the previous example, and we want to replace node $d$ with node $d'$. The computations we'd need to perform are:

$$
r \leftarrow hash(e, hash(c, d)) \\
r' \leftarrow hash(e, hash(c, d'))
$$

Then, as long as $r = g$, and the same values were used for $c$ and $e$ in both computations, we can be convinced that the new root of the tree is $r'$.

The processor accomplishes the above by executing the following instructions:

```
// verify the old merkle path
[MV, RPR]                   // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR     // execute 6 more Rescue rounds
MPV                         // copy result & absorb the next node into the state
RPR RPR RPR RPR RPR RPR RPR // execute 7 Rescue rounds
HOUT                        // return elements 4, 5, 6, 7 of the state as output

// verify the new merkle path
[MU, RPR]                   // init state and execute Rescue round (concurrently)
RPR RPR RPR RPR RPR RPR     // execute 6 more Rescue rounds
MPU                         // copy result & absorb the next node into the state
RPR RPR RPR RPR RPR RPR RPR // execute 7 Rescue rounds
HOUT                        // return elements 4, 5, 6, 7 of the state as output
```

The semantics of `MV` and `MU` instructions are similar to the semantics of `MP` instruction from the previous example (and `MVA` and `MUA` are similar to `MPA`) with one important difference: `MV*` instructions add the absorbed node (together with its index in the tree) to permutation column $p_1$, while `MU*` instructions remove the absorbed node (together with its index in the tree) from $p_1$. Thus, if the same nodes were used during both Merkle path verification, the state of $p_1$ should not change. This mechanism is used to ensure that the same internal nodes were used in both computations.

## AIR constraints

When describing AIR constraints, we adopt the following notation: for column $x$, we denote the value in the current row simply as $x$, and the value in the next row of the column as $x′$. Thus, all transition constraints described in this note work with two consecutive rows of the execution trace.

### Row address constraint

As mentioned above, row address $r$ starts at $0$, and is incremented by $1$ with every row. The first condition can be enforced with a boundary constraint which specifies $r=0$ at the first row. The second condition can be enforced via the following transition constraint:

$$
r' - r - 1 = 0
$$

This constraint should not be applied to the very last row of the hasher execution trace, since we do not want to enforce a value that would conflict with the first row of a subsequent co-processor in the Auxiliary Table. Therefore we can create a special virtual flag for this constraint using the $s_0$ selector column from the Auxiliary Table that selects for the hash co-processor. (This is _not_ one of the hasher's internal selector columns which are desribed above.)

The modified row address constraint which should be applied is the following:

$$
(1 - s_0') \cdot (r' - r - 1) = 0
$$

_Note: this constraint should also be multiplied Auxiliary Table's selector flag $s_0$, as is true for all constraints in this co-processor._

### Selector columns constraints

For selector columns, first we must ensure that only binary values are allowed in these columns. This can be done with the following constraints:

$$
s_0^2 - s_0 = 0 \\
s_1^2 - s_1 = 0 \\
s_2^2 - s_2 = 0
$$

Next, we need to make sure that unless $f_{out}=1$ or $f_{out}'=1$, the values in columns $s_1$ and $s_2$ are copied over to the next row. This can be done with the following constraints:

$$
(s_1' - s_1) \cdot (1 - f_{out}') \cdot (1 - f_{out}) = 0 \\
(s_2' - s_2) \cdot (1 - f_{out}') \cdot (1 - f_{out})= 0
$$

Next, we need to enforce that if any of $f_{abp}, f_{mpa}, f_{mva}, f_{mua}$ flags is set to $1$, the next value of $s_0$ is $0$. In all other cases, $s_0$ should be unconstrained. These flags will only be set for rows that are 1 less than a multiple of 8 (the last row of each cycle). This can be done with the following constraint:

$$
s_0' \cdot (f_{abp} + f_{mpa} + f_{mva} + f_{mua})= 0
$$

Lastly, we need to make sure that no invalid combinations of flags are allowed. This can be done with the following constraints:

$$
k_0 \cdot (1 - s_0) \cdot s_1 = 0
$$

The above constraints enforce that on every step which is one less than a multiple of $8$, if $s_0 = 0$, then $s_1$ must also be set to $0$. Basically, if we set $s_0=0$, then we must make sure that either $f_{hout}=1$ or $f_{sout}=1$.

### Node index constraints

Node index column $i$ is relevant only for Merkle path verification and Merkle root update computations, but to simplify the overall constraint system, the same constraints will be imposed on this column for all computations.

Overall, we want values in the index column to behave as follows:

- When we start a new computation, we should be able to set $i$ to an arbitrary value.
- When a computation is finished, value in $i$ must be $0$.
- When we absorb a new node into the hasher state we must shift the value in $i$ by one bit to the right.
- In all other cases value in $i$ should not change.

A shift by one bit to the right can be described with the following equation: $i = 2 \cdot i' + b$, where $b$ is the value of the bit which is discarded. Thus, as long as $b$ is a binary value, the shift to the right is performed correctly, and this can be enforced with the following constraint:

$$
b^2 - b = 0
$$

Since we want to enforce this constraint only when a new node is absorbed into the hasher state, we'll define a flag for when this should happen as follows:

$$
f_{an} = f_{mp} + f_{mv} + f_{mu} + f_{mpa} + f_{mva} + f_{mua}
$$

And then the full constraint would looks as follows:

$$
f_{an} \cdot (b^2 - b) = 0
$$

Next, to make sure when a computation is finished $i=0$, we can use the following constraint:

$$
f_{out} \cdot i = 0
$$

Finally, to make sure that the value in $i$ is copied over to the next row unless we are absorbing a new row or the computation is finished, we impose the following contraint:

$$
(1 - f_{an} - f_{out}) \cdot (i' - i) = 0
$$

To satisfy these constraints for computations not related to Merkle paths (i.e., 2-to-1 hash and liner hash of elements), we can set $i = 0$ at the start of the computation. This guarantees that $i$ will remain $0$ until the end of the computation.

### Hasher state constraints

Hasher state columns $h_0, ..., h_{11}$ should behave as follows:

- For the first $7$ row of every $8$-row cycle (i.e., when $k_0=0$), we need to apply [Rescue Prime](https://eprint.iacr.org/2020/1143) round constraints to the hasher state. For brevity, we omit these constraints from this note.
- On the $8$th row of every $8$-row cycle, we apply the constraints based on which transition flag is set as described in the table below.

| Condition\_                                      | Constraint                                                                                                      | Description                                                                                                                                                                                                                                                                           |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| $f_{out}=1$                                      | none                                                                                                            | When a computation is completed, the next hasher state is unconstrained.                                                                                                                                                                                                              |
| $f_{abp}=1$                                      | $h'_j - h_j = 0$ for $j \in \{0, ..., 3\}$                                                                      | When absorbing the next set of elements into the state during linear hash computation, the first $4$ elements (the capacity portion) is carried over to the next row.                                                                                                                 |
| $f_{mp}=1$ or <br> $f_{mv}=1$ or <br> $f_{mu}=1$ | $(1 - b) \cdot (h_{j +4}' - h_{j+4})+$ <br> $b \cdot (h_{j + 8}' - h_{j + 4})=0$ <br> for $j \in \{0, ..., 3\}$ | When absorbing the next node during Merkle path computation, the result of the previous hash ($h_4, ..., h_7$) are copied over either to $(h_4', ..., h_7')$ or to $(h_8', ..., h_{11}')$ depending on the value of $b$, which is defined in the same was as in the previous section. |

The above can be translated into a set of constraint straightforwardly like so:

$$
f_{abp} \cdot (h'_j - h_j) = 0 \\
(f_{mp} + f_{mv} + f_{mu}) \cdot \left( (1 - b) \cdot (h_{j +4}' - h_{j+4})+ b \cdot (h_{j + 8}' - h_{j + 4}) \right)=0
$$

### Permutation product constraints

This section describes constraints which enforce updates for permutation product columns $p_0$ and $p_1$. These columns can be updated only on rows which are multiples of $8$ or one less than a multiple of $8$. On all other rows the values in the columns remain the same.

To simplify description of constraints, we define the following variables. Below $\alpha$ and $\beta$ are random values sent by the verifier to the prover after the prover commits to the execution trace described by the main $17$ columns.

$$
m = k_0 + 2 \cdot k_2 + \sum_{j=0}^2 (2^{j+2} \cdot s_j) \\
v_h = \beta + \alpha \cdot m + \alpha^2 \cdot r + \alpha^3 \cdot i \\
v_a = \sum_{j=0}^{3}(\alpha^{j+4} \cdot h_j) \\
v_b = \sum_{j=4}^{7}(\alpha^{j+4} \cdot h_j) \\
v_c = \sum_{j=8}^{11}(\alpha^{j+4} \cdot h_j) \\
v_d = \sum_{j=8}^{11}(\alpha^j \cdot h_j)
$$

In the above:

- $m$ is a _transition label_ which uniquely identifies each transition function.
- $v_h$ is a _common header_ which is a combination of transition label, row address, and node index.
- $v_a$, $v_b$, $v_c$ are the first, second, and third words (4 elements) of the hasher state.
- $v_d$ is the third word in the hasher state, but with the same $\alpha$ coefficients as used for the second word.

Armed with the above notation, we can describe constraints for updating column $p_0$ as follows.

| Condition\_                                      | Constraint                                               | Description                                                                                                                                                                          |
| ------------------------------------------------ | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| $f_{bp}=1$                                       | $p_0' = p_0 \cdot (v_h + v_a + v_b + v_c)$               | When starting a new simple or linear hash computation, the entire hasher state is included into $p_0$.                                                                               |
| $f_{mp}=1$ or <br> $f_{mv}=1$ or <br> $f_{mu}=1$ | $p_0' = p_0 \cdot (v_h + (1-b) \cdot v_b + b \cdot v_d)$ | When starting a Merkle path computation, we include the leaf of the path into $p_0$. The leaf is selected from the state based on value of $b$ (defined as in the previous section). |
| $f_{abp}=1$                                      | $p_0' = p_0 \cdot (v_h + v'_b + v'_c - (v_b + v_c))$     | When absorbing a new set of elements into the state while computing a linear hash, we include deltas between the last $8$ elements of the hasher state (the rate) into $p_0$.        |
| $f_{hout}=1$                                     | $p_0' = p_0 \cdot (v_h + v_b)$                           | When a computation is complete, we include the second word of the hasher state (the result) into $p_0$                                                                               |
| $f_{sout}=1$                                     | $p_0' = p_0 \cdot (v_h + v_a + v_b + v_c)$               | When we want to return the entire state of the hasher, we include the whole state into $p_0$                                                                                         |
| otherwise                                        | $p_0' = p_0$                                             | $p_0$ does not change.                                                                                                                                                               |

We can combine the above constraints into a single expression like so:

$$
p_0' = p_0 \cdot ([(f_{bp} + f_{sout}) \cdot (v_h + v_a + v_b + v_c)] + \\
[(f_{mp} + f_{mv} + f_{mu}) \cdot (v_h + (1-b) \cdot v_b + b \cdot v_d)] + \\
[f_{abp} \cdot (v_h + v'_b + v'_c - (v_b + v_c)] + [f_{hout} \cdot (v_h + v_b)] + \\
1 - (f_{bp} + f_{mp} + f_{mv} + f_{mu} + f_{abp} + f_{out})
)
$$

Note that the degree of the above constraint is $7$.

To describe constraints for column $p_1$, we will change the definition of the _common header_ as follows:

$$
v_g = \beta + \alpha^3 \cdot i
$$

Then, the constraints can be described as follows:

| Condition\_                 | Constraint                                               | Description                                                                                                                        |
| --------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| $f_{mv}=1$ <br> $f_{mva}=1$ | $p_1' = p_1 \cdot (v_g + b \cdot v_b + (1-b) \cdot v_d)$ | When starting a Merkle root update or absorbing a new node into the hasher state, the non-leaf node should be included into $p_1$. |
| $f_{mu}=1$ <br> $f_{mua}=1$ | $p_1' \cdot (v_g + b \cdot v_b + (1-b) \cdot v_d) = p_1$ | When starting a Merkle root update or absorbing a new node into the hasher state, the non-leaf node should be removed from $p_1$.  |
| otherwise                   | $p_1' = p_1$                                             | $p_1$ does not change.                                                                                                             |

We can combine the above constraints into a single expression like so:

$$
p_1' \cdot \left( (f_{mv} + f_{mva}) \cdot (v_g + b \cdot v_b + (1-b) \cdot v_d) + 1 - (f_{mv} + f_{mva}) \right) = \\
p_1 \cdot \left( (f_{mu} + f_{mua}) \cdot (v_g + b \cdot v_b + (1-b) \cdot v_d) + 1 - (f_{mu} + f_{mua}) \right)
$$

Note that the degree of the above constraint is $7$.

Together with boundary constraints enforcing that $p_1=1$ at the first and last rows, the above constraint ensures that if a node was included into $p_1$ as a part of verifying the old Merkle path, the same node must be removed from $p_1$ as a part of verifying the new Merkle path.
