# Range Checker

Miden VM relies very heavily on 16-bit range-checks (checking if a value of a field element is between $0$ and $2^{16}$). For example, most of the [u32 operations](./stack/u32_ops.md), need to perform between two and four 16-bit range-checks per operation. Similarly, operations involving memory (e.g. load and store) require two 16-bit range-check per operation.

Thus, it is very important for the VM to be able to perform a large number 16-bit range checks very efficiently. In this note we describe how this can be achieved using permutation checks.

## 8-bit range checks

First, let's define a construction for the simplest possible 8-bit range-check. This can be done with a single column as illustrated below.

![rc_8_bit_range_check](../assets/design/range/rc_8_bit_range_check.png)

For this to work as a range-check we need to enforce a few constraints on this column:

- Value in the first row must be $0$.
- Value in the last row must be $255$.
- As we move from one row to the next, we can either keep the value the same, or increment it by $1$.

Denoting $v$ as the value of column $v$ in the current row, and $v'$ as the value of column $v$ in the next row, we can enforce the last condition as follows:

$$
(v' - v) \cdot (v' - v - 1) = 0
$$

Together, these constraints guarantee that all values in column $v$ are between $0$ and $255$ (inclusive).

We can then add another column $p_0$, which will keep a running product of values in $v$ offset by random value $\alpha_0$ (provided by the verifier). Transition constraints for column $p_0$ would look like so:

$$
p'_0 - p_0 \cdot (\alpha_0 + v) = 0
$$

Using these two columns we can check if some other column in the execution trace is a permutation of values in $v$. Let's call this other column $x$. We can compute the running product of $x$ in the same way as we compute the running product for $v$. Then, we can check that the last value in $p_0$ is the same as the final value for the running product of $x$ (this is the permutation check).

While this approach works, it has a couple of limitations:

- First, column $v$ must contain all values between $0$ and $255$. Thus, if column $x$ does not contain one of these values, we need to artificially add this value to $x$ somehow (i.e., we need to pad $x$ with extra values).
- Second, assuming $n$ is the length of execution trace, we can range-check at most $n$ values. Thus, if we wanted to range-check more than $n$ values, we'd need to introduce another column similar to $v$.

To get rid of the padding requirement, we can add a _selector_ column, which would contain $1$ for values we want to include in the running product, and $0$ for the ones we don't. But we can address both issues with a single solution.

### A better construction

Let's add two selector column to our table $s_0$ and $s_1$ as illustrated below.

![rc_better_construction](../assets/design/range/rc_better_construction.png)

The purpose of these columns is as follows:

- When $s_0 = 0$ and $s_1 = 0$, we won't include the value into the running product.
- When $s_0 = 1$ and $s_1 = 0$, we will include the value into the running product.
- When $s_0 = 0$ and $s_1 = 1$, we will include two copies of the value into the running product.
- When $s_0 = 1$ and $s_1 = 1$, we will include four copies of the value into the running product.

Thus, for the table pictured below, the running product will include: a single $0$, a single $1$, no $2$'s, and five $3$'s etc.

To keep the description of constraints simple, we'll first define the four flag values as follows:

$$
f_0 = (1 - s_0) \cdot (1 - s_1)
$$

$$
f_1 = s_0 \cdot (1 - s_1)
$$

$$
f_2 = (1 - s_0) \cdot s_1
$$

$$
f_3 = s_0 \cdot s_1
$$

Thus, for example, when $s_0 = 1$ and $s_1 = 1$, $f_3 = 1$ and $f_0 = f_1 = f_2 = 0$.

Then, we'll update transition constraints for $p_0$ like so:

$$
p'_0 - p_0 \cdot \left((\alpha_0 + v)^4 \cdot f_3 + (\alpha_0 + v)^2 \cdot f_2 + (\alpha_0 + v) \cdot f_1 + f_0\right) = 0
$$

The above ensures that when $f_0 = 1$, $p_0$ remains the same, when $f_1 = 1$, $p_0$ is multiplied by $(\alpha_0 + v)$, when $f_2 = 1$, $p_0$ is multiplied by $(\alpha_0 + v)^2$, and when $f_3 = 1$, $p_0$ is multiplied by $(\alpha_0 + v)^4$.

We also need to ensure that values in columns $s_0$ and $s_1$ are binary (either $0$ or $1$). This can be done with the following constraints:

$$
s_0^2 - s_0 = 0
$$

$$
s_1^2 - s_1 = 0
$$

And lastly, for completeness, we still need to impose a transition constraint that we had in the naive approach:

$$
(v' - v) \cdot (v' - v - 1) = 0
$$

This 3-column table addresses the limitations we had as follows:

1. We no longer need to pad the column we want to range-check with extra values because we can skip the values we don't care about.
2. We can support almost $4n$ range checks (when $n$ is relatively large). Though, for short traces (when $n < 256$), we can range-check at most $n$ unique values.

The one downside of this approach is that the degree of our constraints is now $6$ (vs. $2$ in the naive approach). But in the context of Miden VM this doesn't matter as maximum constraint degree for the VM is $9$ anyway.

## 16-bit range checks

To support 16-bit range checks, let's try to extend the idea of the 8-bit table. Our 16-bit table would look like so (the only difference is that column $u$ now has to end with value $65535$):

![rc_16_bit_range_check](../assets/design/range/rc_16_bit_range_check.png)

While this works, it is rather wasteful. In the worst case, we'd need to enumerate over 65K values, most of which we may not actually need. It would be nice if we could "skip over" the values that we don't want. One way to do this could be to add bridge rows between two values to be range checked and add constraints to enforce the consistency of the gap between these bridge rows.

If we allow gaps between two consecutive rows to only be 0 or powers of 2, we could enforce a constraint:

$$
\Delta u \cdot (\Delta u - 1)  \cdot (\Delta u - 2)  \cdot (\Delta u - 4)  \cdot (\Delta u - 8)  \cdot (\Delta u - 16)  \cdot (\Delta u - 32)  \cdot (\Delta u - 64)  \cdot (\Delta u - 128) = 0
$$

This constraint has a degree 9. This construction allows the minimum trace length to be 1024.

We could even go further and allow the gaps between two consecutive rows to only be 0 or powers of 3. In this case we would enforce the constraint:

$$
\Delta u \cdot (\Delta u - 1)  \cdot (\Delta u - 3)  \cdot (\Delta u - 9)  \cdot (\Delta u - 27)  \cdot (\Delta u - 81)  \cdot (\Delta u - 243)  \cdot (\Delta u - 729)  \cdot (\Delta u - 2187) = 0
$$

This allows us to reduce the minimum trace length to 64.

To find out the number of bridge rows to be added in between two values to be range checked, we represent the gap between them as a linear combination of powers of 3, ie,

$$
(r' - r) = \sum_{i=0}^{7} x_i \cdot 3^i
$$

Then for each $x_i$ except the first, we add a bridge row at a gap of $3^i$.

## Miden approach

This construction is implemented in Miden with the following requirements, capabilities, and constraints.

### Requirements

- 3 columns of the main trace: $s_0, s_1, v$.
- 1 [bus](./multiset.md#communication-buses) $b_{range}$ to ensure that the range checks performed in the range checker match those requested by other VM components (the [stack](./stack/u32_ops.md#range-checks) and the [memory chiplet](./chiplets/memory.md)).

### Capabilities

TODO: Update this section
The construction gives us the following capabilities:
- For long traces (when $n > 2^{16}$), we can do over $3n$ arbitrary 16-bit range-checks.
- For short traces ($2^{10} < n \le 2^{16}$), we can range-check at slightly fewer than $n$ unique values, but if there are duplicates, we may be able to range-check up to $3n$ total values.

### Execution trace

The range checker's execution trace looks as follows:

![rc_with_bridge_rows.png](../assets/design/range/rc_with_bridge_rows.png)

The columns have the following meanings:
- $s_0$ and $s_1$ are selector columns that are combined into flags to indicate the number of times the value in that row should be range checked (included into the running product). With these flags, values can be included 0, 1, 2, or 4 times per row in the execution trace. (Values can be included more times by having multiple trace rows with the same value).
- $v$ contains the values to be range checked.
  - These values go from $0$ to $65535$. Values must either stay the same or increase by powers of 3 less than or equal to $3^7$.
  - The final 2 rows of the 16-bit section of the trace must both equal $65535$. The extra value of $65535$ is required in order to [pad the trace](./multiset.md#length-of-running-product-columns) so the [$b_{range}$](#communication-bus) running product bus column can be computed correctly.

### Execution trace constraints

First, we'll need to make sure that all selector flags are binary. This can be done with the following constraints:

> $$
s_0^2 - s_0 = 0 \text{ | degree} = 2
$$

> $$
s_1^2 - s_1 = 0 \text{ | degree} = 2
$$

Then, we need to constrain that the consecutive values in the range checker are either same or differ by powers of 3 less than or equal to $3^7$.

> $$
\Delta v \cdot (\Delta v - 1)  \cdot (\Delta v - 3)  \cdot (\Delta v - 9)  \cdot (\Delta v - 27)  \cdot (\Delta v - 81) \\
\cdot (\Delta v - 243)  \cdot (\Delta v - 729)  \cdot (\Delta v - 2187) = 0 \text{ | degree} = 9
$$

In addition to the transition constraints described above, we also need to enforce the following boundary constraints:

- Value of $v$ in the first row is $0$.
- Value of $v$ in the last row is $65535$.

### Communication bus

$b_{range}$ is the [bus](./multiset.md#communication-buses) that connects components which require 16-bit range checks to the range-checked values in the 16-bit section of the range checker. The bus constraints are defined by the components that use it to communicate.

Requests are sent to the range checker bus by the following components:
- The Stack sends requests for 16-bit range checks during some [`u32` operations](./stack/u32_ops.md#range-checks).
- The [Memory chiplet](./chiplets/memory.md) sends requests for 16-bit range checks against the values in the $d_0$ and $d_1$ trace columns to enforce internal consistency.

Responses are provided by the range checker as follows.

Once again, we'll make use of variable $z$, which represents how a row in the execution trace is reduced to a single value.

$$
z = (\alpha_0 + v)^4 \cdot f_3 + (\alpha_0 + v)^2 \cdot f_2 + (\alpha_0 + v) \cdot f_1 + f_0
$$

Transition constraints for this are fairly straightforward:

> $$
b'_{range} = b_{range} \cdot z \text{ | degree} = 7
$$

If $b_{range}$ is initialized to $1$ and the values sent to the bus by other VM components match those that are range-checked in the the trace, then at the end of the trace we should end up with $b_{range} = 1$.

In addition to the transition constraint described above, we also need to enforce the following boundary constraint:

- The value of $b_{range}$ in the first and last rows is $1$.
