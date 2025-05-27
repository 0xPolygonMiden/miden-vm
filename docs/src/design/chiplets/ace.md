# ACE chip design

The following formalizes the description and functionality of the ACE chiplet. It is still a work in progress, but will evolve to serve as the documentation for its implementation.

TL;DR: The chiplet requires 16 columns and has maximum internal degree 5, which provides more freedom around its placement within the chiplet trace. It requires the caller to make one chiplet bus message for initializing the computation. The input variables and unhashed circuit must be written to a contiguous memory region but allow for multiple evaluations of a circuit with different inputs, keeping the unhashed circuit cached. The design could be optimized in the future if the VM supported read-only memory, though these optimizations would mainly reduce the cycles for the caller.

#### Columns

The ACE chiplet contains 16 internal columns. Cells marked with = behave the same way in each mode. Empty cells are unconstrained and can be set to 0. When a column is unnamed, we refer to the column by its name used within the selected operation

| $s_{start}$ | $s_{block}$ | $ctx$ | $ptr$ | $clk$ |      | $id_0$ | $v_{0,0}$ | $v_{0,1}$ | $id_1$ | $v_{1,0}$ | $v_{1,1}$ |            |           |           | $m_0$ |
|-------------|-------------|-------|-------|-------|------|--------|-----------|-----------|--------|-----------|-----------|------------|-----------|-----------|-------|
| =           | $f_{read}$  | =     | =     | =     |      | =      | =         | =         | =      | =         | =         | $n_{eval}$ |           | $m_1$     | =     |
| =           | $f_{eval}$  | =     | =     | =     | $op$ | =      | =         | =         | =      | =         | =         | $id_2$     | $v_{2,0}$ | $v_{2,1}$ | =     |

The chiplet trace contains multiple sections which each perform a different circuit evaluation. Each section will read through an entire word-aligned memory region containing

- $I$ inputs variables as extension field elements, with two inputs per word.
- $C$ circuit constants, encoded in the same way as the input variables, originating the commitment of the circuit description.
- $N$ circuit instructions, encoded element-by-element representing a triple $(op, id_l, id_r)$.
  For both inputs and constants, it is possible to pad their respective sections with zeros which will be ignored by the circuit instructions.

The caller is responsible for ensuring the input variables are written in the region preceding the unhashed circuit. If the same circuit is evaluated many times, the input region will be overwritten with the inputs for the new evaluation. The caller requests the evaluation of circuit by pushing the chiplet bus messages $(\mathsf{ACE\_LABEL}, ctx, ptr, clk, n_{read}, n_{eval})$, where

- $(ctx, clk)$ is the memory access pair at which every row will read the memory
- $ptr$ is the word-aligned pointer to the region containing the start of the input variables
- $n_{read} = I + C$ is the total number of input and constants read by the chiplet before evaluation
- $n_{eval} = N$ is the number of arithmetic operations performed by the chiplet.

We'll refer to a *section* as a sequence of rows for a single circuit evaluation containing two *blocks* of READ and EVAL operations. A section is initialized by setting the selector $s_{start}$ to 1, and is zero for the remaining rows in the section. Each row reads either a word or element from memory with access $(ctx, ptr, clk)$. The wires identified by $(ctx, clk, id_i, v_{i,0}, v_{i,1})$ for $i=0,1,2$ are updated by either inserting a new node in the evaluation graph with multiplicity $m_i$, or removing it once. The node $i=0$ is always inserted, either because it is read from memory or because it is obtained by evaluating $op \in \{+, \times, -\}$ on nodes 1, 2. The $id$s of inserted nodes are unique, starting at $n_{read} + n_{eval}$ and decreasing by 1 with each insertion. The last node corresponding to the evaluation of the circuit has $id=0$ and must equal 0.

#### Flags and boundary constraints

We start by establishing boundary constraints for the chiplet and derive flags that activate different constraints depending on the operation being performed in the row.

The chiplet trace activates different chiplet constraints using a common set of binary selectors. While it is likely that the ACE chiplet will appear in third position, we derive the flags and boundary constraints for the general case where the chiplet appears in the d-th position. Accounting for this degree allows us to evaluate whether we need a separate degree-1 internal selector for activating the chiplet's constraints. The layout of the chiplet trace will look something like the following.

| Chiplet  | $s_1, \ldots, s_{d-1}$ | $s_{d}$       | ...           |
|----------|------------------------|---------------|---------------|
| Previous | $[1, ..., 1, 0]$       | $cols_{prev}$ | $cols_{prev}$ |
| ACE      | $[1, ..., 1, 1]$       | $0$           | $cols_{ace}$  |
| Next     | $[1, ..., 1, 1]$       | $1$           | $0$           |

From these common selectors, we derive the following binary flags which indicate which portion of the ACE chiplet is active.

- $f_{prev}$: The previous chiplet is active
- $f_{ace}$: The ACE chiplet is active
- $f_{ace, first}'$: Next row is the first row in ACE chiplet
- $f_{ace, next}$: Current and next rows are in ACE chiplet
- $f_{ace, last}$: Last row in ACE chiplet

$$
\begin{aligned}
f_{prev} &\gets (1 - s_{d-1}) \cdot \prod_{i=1}^{d-2} s_{i} && | \deg = d-1\\
f_{ace} &\gets (1 - s_{d}) \cdot \prod_{i=1}^{d-1} s_{i} && | \deg = d\\
f_{ace, first}' &\gets f_{prev} \cdot (1 - s_{d-1}') && | \deg = d \\
f_{ace, next} &\gets f_{ace} \cdot (1 - s_{d}') && | \deg = d + 1\\
f_{ace, last} &\gets f_{ace} \cdot s_{d}' && | \deg = d + 1\\
\end{aligned}
$$

#### Section constraints

The selector $s_{start}$ indicates the start of a new section, from which we can derive the following flags indicating which part of the section the current row is in:

- $f_{start}$ : the current row initializes the section
- $f_{next}$: the current and next rows are in the same section
- $f_{end}$: the current row finalizes the section
  $$
  \begin{aligned}
  f_{start} &\gets f_{ace} \cdot s_{start} && | \deg = d+1\\
  f_{next} &\gets f_{ace, next} \cdot (1 - s_{start}')  && | \deg = d+2\\
  f_{end} &\gets f_{ace, next} \cdot s_{start}' + f_{ace,last} && | \deg = d+2\\
  \end{aligned}
  $$
  These flags require the following constraints on $s_{start}$, namely
- it is binary,
- it must equal 1 in the first row,
- it must equal 0 in the last row,
- two consecutive rows cannot initialize a section, therefore a section contains at least two rows
  $$
  \begin{aligned}
  f_{ace} \cdot s_{start} \cdot (1 - s_{start}) &= 0 && | \deg = d + 2\\
  f_{ace, first}' \cdot (1 - s_{start}') &= 0 && | \deg = d + 1\\
  f_{ace, last} \cdot s_{start} &= 0 && | \deg = d + 2\\
  f_{ace, next} \cdot s_{start} \cdot s_{start}' &= 0 && | \deg = d + 2\\
  \end{aligned}
  $$

A section is composed of a READ block followed by an EVAL block. The flag indicating which block is active is derived from the binary selector $s_{block}$. The constraint ensures they are mutually exclusive
$$
\begin{aligned}
f_{read} \gets (1-s_{block}) & &&| \deg = 1\\
f_{eval} \gets s_{block} & &&| \deg = 1\\
\\
f_{ace} \cdot (1-s_{block}) \cdot s_{block} = 0 && &| \deg = d + 2\\
\end{aligned}
$$

The following constraints ensure the proper layout of the trace. In particular, it contains one or more sections each with consecutive READ and EVAL blocks.

- The first row cannot be EVAL, so it must be READ
- A row after EVAL cannot be read
- The last cannot be READ, so it must be EVAL
  $$
  \begin{aligned}
  f_{start} \cdot f_{eval} &= 0 && | \deg = d + 2\\
  f_{next} \cdot f_{eval} \cdot f_{read}' &= 0 && | \deg = d + 4\\
  f_{end} \cdot f_{read} &= 0 && | \deg = d + 3\\
  \end{aligned}
  $$
  In particular, we can infer from the above that
- Each section contains at least two rows (a READ and EVAL)
- A row following a READ is always in the same block.

When the EVAL block starts, the next $id_0$ is expected to be equal to the number of evaluation nodes $n_{eval}$ provided by the caller at initialization. Therefore, we have to ensure that it is propagated across the READ rows and corresponds to $id_0$ in the first EVAL row.
$$
f_{ace} \cdot f_{read} \cdot
\big[f_{read}' \cdot n_{eval}' + f_{eval}' \cdot id_0' - n_{eval}\big] = 0 \quad | \deg = d + 3.
$$

The transition constraints for the variables are the following

- Across the section, $ctx$ and $clk$ are constant.
- A READ/EVAL block requests a word/element from memory, so the $ptr$ increases by 4/1, respectively.
- A READ/EVAL block adds 2/1 new nodes to the evaluation graph, so $id_0$ decreases by that amount.
  $$
  \begin{aligned}
  f_{next} \cdot (ctx' - ctx) &= 0 && | \deg = d + 3\\
  f_{next} \cdot (clk' - clk) &= 0 && | \deg = d + 3\\
  f_{next} \cdot \big[ptr' - ptr + 4 \cdot f_{read} + f_{eval}\big] &= 0 && | \deg = d + 3\\
  f_{next} \cdot \big[id_0 - id_0' + 2 \cdot f_{read} + f_{eval}\big] &= 0 && | \deg = d + 3\\
  \end{aligned}
  $$

#### Bus messages

The chiplet bus is used both for receiving the initialization message in the first row and the memory read requests in all rows. These are

- $(\mathsf{ACE\_LABEL}, ctx, ptr, clk, n_{read}, n_{eval})$, when $f_{start} = 1$ and
- $(\mathsf{MEMORY\_READ\_WORD\_LABEL}, ctx, ptr, clk, v_{0,0}, v_{0,1}, v_{1,0}, v_{1,1})$, when $f_{read} = 1$,
- $(\mathsf{MEMORY\_READ\_ELEMENT\_LABEL}, ctx, ptr, clk, instr)$, when $f_{eval} = 1$

The values are obtained as-is from the current row, except for the two following degree-1 expressions.

- $n_{read} \gets id_0 - n_{eval}$, since in the first row, $id_0$ is expected to be equal to the total number of nodes inserted
- $instr \gets id_0 + id_1 \cdot 2^{30} + (op+1)\cdot 2^{60}$, which encodes a circuit instruction performing $op \in \{- ,\times, +\}$ to the nodes $id_1, id_2 \in [0, 2^{30}[$ .

We'll forgo the concrete expressions for the corresponding bus messages reduced by challenges $\alpha_0, \alpha_1, \ldots$, and assume they are given by degree-1 expressions $u_{chip}, u_{mem, read}, u_{mem, eval}$.

Given access to the auxiliary bus column $b$, the constraint applied is
$$
f_{ace} \cdot  b' \cdot \Big( f_{read}\cdot w_{mem,read} + f_{eval}\cdot w_{mem,eval} \Big) - b \cdot \Big(f_{ace} + f_{start}\cdot (w_{chip,start} - 1)\Big) = 0 \quad | \deg = d+3.
$$

##### Wire bus

Each row of the chip can make 3 requests to the circuit's wire bus. For $i = 0, 1, 2$, each message in the request has the form $(ctx, clk, id_i, v_{i,0}, v_{i,1})$, which uniquely identifies a node in the DAG representing the evaluation of the circuit. Sending this message to the bus can be viewed as updating the total degree of the node in the graph. When performing a READ operation, a node is added to the graph, and we set its degree update $e_i$ to be equal to the total number of outgoing edges it will have by the end of the evaluation. This value is also referred to as the *multiplicity* $m_i$. When a node is used as an argument of an arithmetic operation, we would set $e_i = - 1$.

The expression $e_i$ is derived from $m_i$ and the operation flag, so that the wire bus update is uniform across all rows of the chiplet's trace.

- $v_0$ always defines a new node, and each operation defines its identifier $id_0$ and multiplicity $m_0$ using the same columns.
  $$
  e_0 \gets  m_0 \quad \text{| degree} = 1
  $$
- $v_1$ defines a new node when the operation is a READ, but is an input during an EVAL. Again, the columns for these values are identical.
  $$
  e_1 \gets  f_{read} \cdot m_1 - f_{eval} \quad \text{| degree} = 2
  $$
- $v_2$ is unused during a READ, and an input during EVAL
  $$
  e_2 \gets - f_{eval} \quad \text{| degree} = 1
  $$

The auxiliary logUp bus column $b$ is updated as follows. Given random challenges $\alpha_j$ for $j = 0, ..., 5$,  
let $w_i = \alpha_0 + \alpha_1 \cdot ctx + \alpha_2 \cdot clk + \alpha_3 \cdot id_i + \alpha_4 \cdot v_{i,0} + \alpha_5 \cdot v_{i,1}$ be the randomized node value. The value of the bus in the next column is given by
$$
b' = b + \sum_{i=0}^2 \frac{e_i}{w_i},
$$

The actual constraint is given by normalizing the denominator
$$
f_{ace}\cdot \left( (b - b') \cdot \prod_{i=0}^{2}w_i + \left(e_0 \cdot w_1 \cdot w_2 + e_1 \cdot w_0 \cdot w_2 + e_2 \cdot w_0 \cdot w_1\right)\right) = 0 \quad \text{| degree} = d + 4.
$$

#### READ block

In a READ block, each row requests a row from memory a word containing two extension field elements $v_0 = (v_{0,0}, v_{0,1})$ and $v_1 = (v_{1,0}, v_{1,1})$. We have already described how these are then added to the wire bus. The only constraint we enforce is that $id_0$ and $id_1$ are consecutive

$$
\begin{aligned}
f_{ace} \cdot f_{read} \cdot (id_1 - id_0 + 1)  &= 0 && | \deg = d + 2\\
\end{aligned}
$$

#### EVAL block

An EVAL block checks that the arithmetic operation $op$ was correctly applied to inputs $v_1, v_2$ and results in $v_0$. The result is given by the degree-4 expression
$$
v_{out} \gets op^2 \cdot \big[ v_1 + op\cdot v_2 \big] + (1 - op^2)  \cdot \big[ v_1 \cdot v_2 \big]
= \begin{cases}
v_1 - v_2, & op = -1, \\
v_1 \times v_2, & op = 0, \\
v_1 + v_2, & op = 1. \\
\end{cases}
$$
The constraints ensuring the operation is correct are applied in every row of the chip

- $op \in \{-1, 0, 1\}$
- $v_0$ is equal to $v_{out}$

$$
\begin{aligned}
f_{ace} \cdot f_{eval} \cdot op \cdot (op^2 - 1) &= 0 && | \deg = d + 4\\
f_{ace} \cdot f_{eval} \cdot (v_0 - v_{out}) &= 0 && | \deg = d + 5\\
\end{aligned}
$$

The actual instruction is given by the field element $instr$ read from memory. It encodes

- the operation $op$ using 2 bits
- the ids of $v_1$ and $v_2$ using 30 bits each and are packed as
  $$
  instr \gets id_0 + id_1 \cdot 2^{30} + (op+1)\cdot 2^{60}.
  $$

It is clear from the constraint on $op$ that $op+1$ will always require 2 bits. Range constraints on $id_1, id_2$ are unnecessary. These ids are sent as-is to the wire bus with multiplicity $-1$. For the logUp argument to be valid, the section must include a row where a node is added to the circuit the same id such that the pole $\frac{-1}{w_i}$ can be annihilated. The only way to do so is if there exists a corresponding $id_0$ matching the one in the instruction. This is ensured by the pointers given by the starting and ending chiplet messages, and the constraint enforcing it to be strictly increasing in each row. Therefore, as long as the trusted circuit contains fewer than $2^{30}$ ids, the $id_1$ and $id_2$ values can never overflow this bound.

To ensure the circuit has finished evaluating and that the final output value is 0, we add the following section boundary constraints to enforce that the section ends with the node with $id_0 = 0$ has value $v_0 = 0$.
$$
\begin{aligned}
f_{end} \cdot id_0 &= 0 && | \deg = d + 3\\
f_{end} \cdot v_0 &= 0 && | \deg = d + 3\\
\end{aligned}
$$

#### Example

The following is a section of the trace representing the evaluation of the expressions
$$
(s \times (1-s)) \times \alpha + s \cdot (t - 2) - q \cdot (x^n - 1)
$$

With the following inputs stored in memory in addresses `0x0000-0x0008`
$$
\begin{aligned}
v_{16} &= \alpha &
v_{15} &= x^n \\
v_{14} &= s &
v_{13} &= t \\
v_{12} &= q &
v_{11} &= \cdots \\
\end{aligned}
$$
Followed by the constant at `0x000c`
$$v_{10} = 1 \quad v_{9} = 2$$
And the remaining instructions computing the evaluations
$$
\begin{aligned}
v_8 &= v_{10} - v_{14} &&= 1 - s \\
v_7 &= v_{14} \times v_{8} &&= s\times (1-s) \\
v_6 &= v_{13} - v_{9} &&= t - 2 \\
v_5 &= v_{14} \times v_{6} &&= s \times (t - 2) \\
v_4 &= v_{15} - v_{10} &&= x^n - 1 \\
v_3 &= v_{12} \times v_{4} &&= q \times (x^n - 1)  \\
v_2 &= v_{7} \times v_{16} &&= (s\times (1-s)) \times \alpha \\
v_1 &= v_2 + v_5 &&= \Big(\big(s\times (1-s)\big) \times \alpha\Big) + \Big(s \times (t - 2)\Big) \\
v_0 &= v_1 - v_3 &&= \Bigg(\Big(\big(s\times (1-s)\big) \times \alpha\Big) + \Big(s \times (t - 2)\Big)\Bigg) - \Bigg(q \times (x^n - 1)\Bigg)\\
\end{aligned}
$$

| $s_{start}$ | $s_{block}$ | $ctx$ | $ptr$  | $clk$ | $op$     | $id_0$ | $v_{0}$           | $id_1$ | $v_{1}$        | $n_{eval}$/$id_{2}$ | $m_1$/$v_2$  | $m_0$        |
|-------------|-------------|-------|--------|-------|----------|--------|-------------------|--------|----------------|---------------------|--------------|--------------|
| 1           | 0           | ctx   | 0x0000 | clk   |          | 16     | $v_{16} = \alpha$ | 15     | $v_{15} = x^n$ | 8                   | $m_{15} = 1$ | $m_{16} = 1$ |
| 0           | 0           | ctx   | 0x0004 | clk   |          | 14     | $v_{14} = s$      | 13     | $v_{13} = t$   | 8                   | $m_{13} = 1$ | $m_{14} = 3$ |
| 0           | 0           | ctx   | 0x0008 | clk   |          | 12     | $v_{12} = q$      | 11     | -              | 8                   | $m_{11} = 0$ | $m_{12} = 1$ |
| 0           | 0           | ctx   | 0x000c | clk   |          | 10     | $v_{10} = 1$      | 9      | $v_{9} = 2$    | 8                   | $m_{9} = 1$  | $m_{10} = 2$ |
| 0           | 1           | ctx   | 0x0010 | clk   | $-$      | 8      | $v_{8}$           | 10     | $v_{10}$       | 14                  | $v_{14}$     | 1            |
| 0           | 1           | ctx   | 0x0011 | clk   | $\times$ | 7      | $v_{7}$           | 14     | $v_{14}$       | 8                   | $v_{8}$      | 1            |
| 0           | 1           | ctx   | 0x0012 | clk   | $-$      | 6      | $v_{6}$           | 13     | $v_{13}$       | 9                   | $v_{9}$      | 1            |
| 0           | 1           | ctx   | 0x0013 | clk   | $\times$ | 5      | $v_{5}$           | 14     | $v_{14}$       | 6                   | $v_{6}$      | 1            |
| 0           | 1           | ctx   | 0x0014 | clk   | $-$      | 4      | $v_{4}$           | 15     | $v_{15}$       | 10                  | $v_{10}$     | 1            |
| 0           | 1           | ctx   | 0x0015 | clk   | $\times$ | 3      | $v_{3}$           | 12     | $v_{12}$       | 4                   | $v_{4}$      | 1            |
| 0           | 1           | ctx   | 0x0016 | clk   | $\times$ | 2      | $v_{2}$           | 7      | $v_{7}$        | 16                  | $v_{16}$     | 1            |
| 0           | 1           | ctx   | 0x0017 | clk   | $+$      | 1      | $v_1$             | 2      | $v_{2}$        | 5                   | $v_{5}$      | 1            |
| 0           | 1           | ctx   | 0x0018 | clk   | $-$      | 0      | $v_0$             | 1      | $v_{1}$        | 3                   | $v_{3}$      | 0            |

