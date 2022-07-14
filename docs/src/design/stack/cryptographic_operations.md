# Cryptographic operations
In this section we describe the AIR constraint for Miden VM cryptographic operations.

## RPPERM

`RPPERM` operation applies rescue prime permutation to the top $12$ elements of the stack. The stack is assumed to be arranged so that the 8 elements of the rate are at the top of the stack. The capacity word follows, with the number of elements to be hashed at the deepest position in stack. The diagram below illustrates this graphically.

![rpperm](../../assets/design/stack/cryptographic_operations/RPPERM.png)

To facilitate this operation, we will need to perform a lookup in a table described [here](https://maticnetwork.github.io/miden/design/aux_table/hasher.html). 

Let's define a few intermediate variables to simplify constraint description:

$$
state_{pre} = \sum_{j=0}^{11}\alpha_{j+3} \cdot s_i
$$

$$
state_{post} = \sum_{j=0}^{11}\alpha_{i+3} \cdot s_i'
$$

In the above, $state_{pre}$ and $state_{post}$ can be thought of as component of initial and final stack state value post the rescue prime permutation in the permutation check calculation.

$$
v_{input} = \left(\alpha_0 + \alpha_1 \cdot 3+ \alpha_2 \cdot i + state_{pre}\right)
$$

$$
v_{output} = \left(\alpha_0 + \alpha_1 \cdot 9 + \alpha_2 \cdot (i + 7) +  state_{post}\right)
$$

Here, $v_{input}$ and $v_{output}$ can be think of as the component which are multiplied in the hasher co-processor to the $b_{aux}$ at the start and end of the operation during permutation check. 

The prover will also need to provide a non-deterministic ‘helper’ for the lookup. The lookup in the table can be accomplished by including the following values into the lookup product:

> $$
b_{aux}' \cdot v_{input} \cdot v_{output} = b_{aux} \text{ | degree } = 3
$$

where $b_{aux}$ is the running product column of auxiliary table, $i$ is the row address of the execution trace at which the rescue prime permutation started and $\alpha_0$, $\alpha_1$, $\alpha_2$ etc... are random values sent from the verifier to the prover for use in permutation checks. 

The $3$ & $8$ in the permutation check is the unique identifier of hasher `LINEAR_HASH` & `RETURN_STATE` operation which has been explained [here](../stack/unique_identifier.md#identifiers).

The `RPPERM` operation will not change the depth of the stack i.e. the stack doesn't shift while transitioning. The maximum degree of the above operation is $3$.


## MPVERIFY

`MPVERIFY` computes a root of a merkle path for a given specified input node. The stack is assumed to be arranged in such a way the top two elements are depth and index respectively followed by node's value and root of the tree. This operation first fetches a merkle path from the advice provider for the specified root and node index and the path is expected to be at a specific depth. Hashes co-processor is then called to compute a new merkle root of this fetched computed path which is saved in the stack by overwriting the node value section in the stack. Rest all the elements in the stack remains untouched. The diagram below illustrates this graphically.

![mpverify](../../assets/design/stack/cryptographic_operations/MPVERIFY.png)

To facilitate this operation, we will need to perform a lookup in a table described [here](https://maticnetwork.github.io/miden/design/aux_table/hasher.html). 

Let's define a few intermediate variables to simplify constraint description:

$$
state_{pre} = \sum_{j=0}^{11}\alpha_{j+3} \cdot s_i
$$

$$
state_{post} = \sum_{j=0}^{11}\alpha_{i+3} \cdot s_i'
$$

$$
v_{input} = 
$$

In the above, $v_{new}$ and $v_{old}$ can be thought of as component of new and old memory value (whole word) in the permutation check calculation. 

The prover will also need to provide a non-deterministic ‘helper’ for the lookup. The lookup in the table can be accomplished by including the following values into the lookup product:

$$
b_{aux}' \cdot \left(\alpha_0 + \alpha_1 \cdot 11 + \alpha_2 \cdot h_0 + \alpha_3 \cdot s_1 + \sum_{i=0}^3 \alpha_{i+4} \cdot s_{i+2} + \sum_{i=0}^3\alpha_{i + 8} \cdot h_{i+2}\right) \cdot \\
\left(\alpha_0 + \alpha_1 \cdot 1 + \alpha_2 \cdot (h_0 + h_1 \cdot 8 - 1) + \sum_{i=0}^3 \alpha_{i+3} \cdot s_{i+2}'\right) = b_{aux} \\
s_i' - s_{i+4}' = 0 \space where \space i \in \{2, 3, 4, 5\} 
$$

where $b_{aux}$ is the running product column of auxiliary table, $h_0$ is the row address of the execution trace at which the computation of building merkle root started in the hasher, $h_1$ is the length of the merkle path vector fetched from advice provider, $h_2$..$h_5$ is the first word(group of four consecutive elements) of the merkle path and $\alpha_0$, $\alpha_1$, $\alpha_2$ etc... are random values sent from the verifier to the prover for use in permutation checks. 
The value of selector flag of `hasher` co-processor is $0$ and the internal selector flag of `MP_VERIFY` and `RETURN_HASH` are $1, 0, 1$ and $0, 0, 0$ respectively. The bitwise aggregation of these flags with the `hasher` selector flag will come out to be $10 \space and \space 0$ respectivly. On adding one to both the aggregated flag value, we get the unique constant of $11 \space and \space 1$ for `MP_VERIFY` and `RETURN_HASH` respectively. We need these unique value in the grand product to ensure that the value we are looking up from the lookup table is indeed from a `hasher` operation and not from somewhere else.

The `MPVERIFY` operation will not change the depth of the stack i.e. the stack doesn't shift while transitioning. The maximum degree of the above operation is $3$.

## MRUPDATE

`MRUPDATE` operation computes a new root of a merkle tree where a leaf at the specified index is updated to the specified value. The stack is assumed to be arranged in such a way the top two elements are depth and index respectively followed by old node value, new node value and the current root of the tree. This operation updates the leaf node at the specified index in the advice provider with the specified root, and get the merkle path of the leaf. This merkle path is then provided to hasher along with old and new node value to get the new root of the tree. The old and new root of the tree are saved in the stack by overwriting old and new node value in the stack respectively. Rest all the elements in the stack remains untouched. The diagram below illustrates this graphically.

![mrupdate](../../assets/design/stack/cryptographic_operations/MRUPDATE.png)

To facilitate this operation, we will need to perform a lookup in a table described [here](https://maticnetwork.github.io/miden/design/aux_table/hasher.html). The prover will also need to provide a non-deterministic ‘helper’ for the lookup. The lookup in the table can be accomplished by including the following values into the lookup product:

$$
b_{aux}' \cdot \left(\alpha_0 + \alpha_1 \cdot 11 + \alpha_2 \cdot h_0 + \alpha_3 \cdot s_1 + \sum_{i=0}^3 \alpha_{i+4} \cdot s_{i+2} + \sum_{i=0}^3\alpha_{i + 8} \cdot h_{i+2}\right) \cdot \\
\left(\alpha_0 + \alpha_1 \cdot 1 + \alpha_2 \cdot (h_0 + h_1 \cdot 8 - 1) + \sum_{i=0}^3 \alpha_{i+3} \cdot s_{i+2}'\right) \cdot \\
\left(\alpha_0 + \alpha_1 \cdot 11 + \alpha_2 \cdot (h_0 + h_1 \cdot 8) + \alpha_3 \cdot s_1 + \sum_{i=0}^3 \alpha_{i+4} \cdot s_{i+6} + \sum_{i=0}^3\alpha_{i + 8} \cdot h_{i+2}\right) \cdot \\
\left(\alpha_0 + \alpha_1 \cdot 1 + \alpha_2 \cdot (h_0 + h_1 \cdot 16 - 1) + \sum_{i=0}^3 \alpha_{i+3} \cdot s_{i+6}'\right) = b_{aux}\\
s_i' - s_{i+8}' = 0 \space where \space i \in \{2, 3, 4, 5\} 
$$

where $b_{aux}$ is the running product column of auxiliary table, $h_0$ is the row address of the execution trace at which the computation of building merkle root started in the hasher, $h_1$ is the length of the merkle path vector fetched from advice provider, $h_2$..$h_5$ is the first word(group of four consecutive elements) of the merkle path and $\alpha_0$, $\alpha_1$, $\alpha_2$ etc... are random values sent from the verifier to the prover for use in permutation checks. 
The value of selector flag of `hasher` co-processor is $0$ and the internal selector flag of `MP_VERIFY` and `RETURN_HASH` are $1$, $0$, $1$ and $0$, $0$, $0$ respectively. The bitwise aggregation of these flags with the `hasher` selector flag will come out to be $10$ and $0$ respectivly. On adding one to both the aggregated flag value, we get the unique constant of $11$ and $1$ for `MP_VERIFY` and `RETURN_HASH` respectively. We need these unique value in the grand product to ensure that the value we are looking up from the lookup table is indeed from a `hasher` operation and not from somewhere else.

The `MRUPDATE` operation will not change the depth of the stack i.e. the stack doesn't shift while transitioning. The maximum degree of the above operation is $5$.