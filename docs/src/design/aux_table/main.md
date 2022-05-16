# Auxiliary Table

The Auxiliary Table reduces the number of columns required by the execution trace by stacking the execution traces of 4 co-processors that are expected to generate significantly fewer rows than the other VM processors (the decoder, stack, and range checker).

## Auxiliary Table segments

The co-processors in the auxiliary table are:

- [Hash Processor](./hasher.md) (17 columns; degree 8)
- Bitwise & Power of Two Processor, which combines 2 co-processors:
  - [Bitwise Processor](./bitwise.md) (13 columns; degree 6)
  - [Power of Two Processor](./pow2.md) (12 columns; degree 3)
- [Memory Processor](./memory.md) (14 columns; degree 6)

Each co-processor is identified by a set of selector columns which identify its segment in the auxiliary table and cause its constraints to be selectively applied.

During the finalization of the overall execution trace, each co-processor's trace and selector columns are appended to the Auxiliary Table one after another, such that when one co-processor trace ends the trace of the next co-processor starts in the subsequent row.

Additionally, a padding segment must be added to the Auxiliary Table so that the number of rows in the table always matches the overall trace length of the other VM processors, regardless of the length of the co-processor traces. The padding will simply contain zeroes.

## Design Requirements

- The minimum width of the Auxiliary Table is 18 columns, which will fit the Hash processor and 1 selector column to select it.
- The maximum constraint degree for the VM is 9.

### Hasher

- The hasher's constraints are already degree 8, so we are restricted to a 1-degree selector flag.
- The hasher requires 17 columns, so its selector flag must require a single column to keep our aux table to the minimum.
- Each hash operation in the hash processor is performed in an 8-row cycle that must begin at a row number equal to $0\mod8$, so the hash processor's trace must begin on a row number equal to $0\mod8$.
- As described in the Challenge section below, the degree-2 row address transition constraint must not be applied to the last row.

### Bitwise

- The constraints for the bitwise co-processor have degree 6, so its selector flag cannot exceed degree 3.
- The bitwise co-processor requires 13 columns, so it can have at most 5 selector columns.
- Each bitwise operation in the bitwise processor is performed in an 8-row cycle that must begin at a row number equal to $0\mod8$, so the bitwise processor's trace must begin on a row number equal to $0\mod8$.

Note: If the bitwise co-processor is [refactored](https://github.com/maticnetwork/miden/issues/120) to process only one bitwise operation, rather than three, then its internal selector flags could be removed. In this case, the constraint degree would be reduced to 4 and the number of columns would be reduced to 11.

### Power of two

- The constraints for the power of two co-processor have degree 3, so its selector flag cannot exceed degree 6.
- The power of two co-processor requires 12 columns, so it can have at most 6 selector columns.
- The power of two operations is completed in an 8-row cycle that must begin at a row number equal to $0\mod8$, so the Power of Two processor's trace must begin on a row number equal to $0\mod8$.
- As described in the Challenge section below, the input and output aggregation constraints must not be applied to the last row.

### Memory

- The constraints for the memory co-processor have degree 6, so its selector flag cannot exceed degree 3.
- The memory co-processor requires 14 columns, so it can have at most 4 selector columns.
- As described in the Challenge section below, the transition constraints (degree 6) must not be applied to the last row.

## Auxiliary Table co-processor order

For simplicity, all of the "cyclic" co-processors which operate in multi-row cycles and require starting at particular row increments should come before any non-cyclic co-processors, and these should be ordered from longest-cycle to shortest-cycle. This will allow us to avoid any additional alignment padding between co-processors.

To fulfill the requirements above, we'll start by placing the Hasher at the top of the Auxiliary Table with a single selector column beside it where $s_0 = 0$ selects the Hasher. The third requirement for the hasher can easily be resolved with a virtual flag excluding the last row, since the row address constraint is only degree 2.

![](https://i.imgur.com/40eKeih.png)

Next, we would like to include the other cyclic co-processors: the Bitwise and Power of Two processors.

That would leave the Memory co-processor to go last. However, if we use a selector column for each of 4 co-processors and also put the Memory co-processor at the end, then the selectors will cause us to exceed the maximum degree for the Memory co-processor's constraints.

We can get around this problem by combining the Bitwise and Power of Two processors into a single co-processor with a shared trace, so that the Power of Two processor becomes an additional operation in the Bitwise processor which will be selected by the Bitwise processor's internal selector columns. We'll need to add one additional column to the Bitwise trace in order for the Power of Two operation to fit. The degree of the combined processor will be 6, and the selector flag from the two selector columns will push it to degree 8, which is fine.

![](https://i.imgur.com/TQ25pFR.png)

Finally, we come to the Memory co-processor, where we still need to deal with the "last row problem" (described below). The three selector flags for the Memory section mean that the constraint degree is already at the maximum of 9, which gives us 2 options:

1. Put the memory co-processor at the end of the auxiliary table.
2. Modify the transition constraint format.

For now, we'll place the Memory co-processor last after the padding to keep the implementation simple.

![](https://i.imgur.com/1DoTUih.png)

## Auxiliary Table constraints

The auxiliary table needs to enforce constraints on the 3 selector columns that are used to specify the various co-processors to ensure they are binary. The constraints themselves are selectively applied, since two of the columns do not act as selectors for the entire trace. We can enforce this with the following constraints:

$$s_0^2 - s_0 = 0$$
$$s_0 \cdot (s_1^2 - s_1) = 0$$
$$s_0 \cdot s_1 \cdot (s_2^2 - s_2) = 0$$

### Combined Bitwise & Power of Two Co-processor

Because the Bitwise and Power of Two co-processors have been combined to share a trace, the enforcement of operation selectors should be handled at the shared co-processor level rather than individually by the Bitwise or Power of Two co-processors themselves.

The selectors for each operation are as follows:

- `U32AND`: $s_0 = 0$, $s_1 = 0$
- `U32OR`: $s_0 = 0$, $s_1 = 1$
- `U32XOR`: $s_0 = 1$, $s_1 = 0$
- `POW2`: $s_0 = 1$, $s_1 = 1$

The constraints must require that the selectors be binary:
$$s_0^2 - s_0 = 0$$
$$s_1^2 - s_1 = 0$$

## Challenge: the last row problem

Handling transition constraints in the auxiliary table is problematic when applying them based on the a co-processor's set of "selector flags" would cause the first row of the following co-processor's trace to be constrained to a value which is incorrect for the new co-processor.

Let's consider a simple example:

- Let Processor A and Processor B be two co-processors whose execution traces are stacked such that they share the same columns and the trace rows for B start immediately after the trace rows for A end.
- Let $s_0$ be a selector column. When $s_0 = 0$, we apply the constraints for our Processor A. When $s_0 = 1$ we apply the constraints for Processor B.
- Let $a$ be a column whose value should start at 0 in the first row of a processor's trace and be incremented by 1 with each new row within the processor's trace.

In the normal case where the entire length of a set of trace columns is devoted to a single processor, we can use the following constraint for column $a$:

$$not(s_0) * (a' - (a + 1)) = 0$$

However, once we begin stacking co-processors within the same set of columns, we run into an issue where the final enforcement of this transition constraint causes the value of column $a$ in the first row of the subsequent co-processor to be an incorrect value.

For example, if Processor A has 4 rows in its trace, then in the 5th row of the stacked trace, the execution trace of Processor B will start, and the value of $a$ should be reset to 0.

| $s_0$ | $a$                       |
| ----- | ------------------------- |
| 0     | 0                         |
| 0     | 1                         |
| 0     | 2                         |
| 0     | 3                         |
| 1     | CONFLICT: must be 4 and 0 |

CONFLICT: our transition constraint from Processor A will require that this be 4, but Processor B will require that it be 0.

### Affected co-processor constraints

- [Hasher](./hasher.md) - the row address constraint (degree 1):
  $$r' - r - 1 = 0$$
- [Memory](./memory.md) - all transition constraints, in particular this degree 6 constraint:
  $$(1−n_0)⋅(n_1^2−n_1)=0$$
- [Power of 2](./pow2.md) - the input aggregation constraint (degree 2) and the output aggregation constraint (degree 4), which are:
  $$a' - (a_0' + a_1'+a_2'+ ... + a_7' + k_1 \cdot a) = 0$$

$$z' - (\sum\limits_{i=0}^8 p' \cdot t_i' \cdot 2^i + k_1 \cdot z) = 0$$

### Possible solutions

#### Virtual flag to identify the last row of a co-processor:

We could use a virtual flag to prevent these transition constraints from being applied to the final row. However, this will increase the degree of constraints by one, so it can only be used in cases where the degree of the co-processor's constraints plus the degree of the selector flags is <= 8 (since 9 is the maximum constraint degree).

#### Additional column:

Add an extra column to the execution trace of the affected processor. Set the value to 1 for the last row and 0 otherwise. Update the processor's constraints to avoid affecting the degree.

#### Put the affected co-processor last

An affected co-processor can be put at the bottom of the auxiliary table where we won't care about the final transition being enforced.

#### Modified constraint format:

Define the co-processor's transition constraint selector flag to never enforce against the last row.

- Let Processor A, Processor B, and Processor C be three co-processors whose execution traces are stacked such that they share the same columns, the trace rows for B start immediately after the trace rows for A end, and the trace rows for C start immediately after the end of the trace rows for B.
- Let $s_0$ and $s_1$ be selector columns. When $s_0 = 0$, we apply the constraints for Processor A. When $s_0 = 1$ and $s_1 = 0$ we apply the constraints for Processor B. When $s_0 = 1$ and $s_1$ = 1 we apply the constraints for Processor C.

Multiplying Processor A's constraints by $not(s_0')$ instead of by $not(s_0)$ will cause them to stop being enforced after the second-to-last row. For Processor B, this flag would look like $s_0 * not(s_1')$.

This resolves our last-row issue, but it introduces a problem with the first row of a processor's trace.

- If this construction is used for the first processor in the auxiliary table, then the very first row won't be constrained properly unless a validity constraint is defined for that specific row.
- When used for subsequent processors, the following scenario is possible:

  | row # | $s_0$ | $s_1$          | the rest of the trace segment                 |
  | ----- | ----- | -------------- | --------------------------------------------- |
  | i     | 0     | Processor A... | Processor A...                                |
  | i + 1 | 1     | 1              | Processor B transition constraints applied... |
  | i + 2 | 1     | 0              | All Processor B constraints applied...        |

**PROBLEM:** Note that in row `i + 1` the selector flags actually match Processor C, which would result in the following mismatch:

1. The transition constraints for Processor B would be applied
2. Any single-row validity constraints (such as a restriction to binary values) would be applied for Processor C

To solve these issues for the general case, we can take the following approach to constraints:

1. Define different flags for "validity constraints" (enforced on a single row) and transition constraints (enforced between rows). The flag for validity constraints will simply be the combination of selector columns for the processor (e.g. $s_0 * not(s_1)$ for Processor B), while the flag for the transition constraints will be the one described above.
2. Add constraints to the entire auxiliary table trace to enforce that when these columns are acting as selectors they can’t change from 1 -> 0 (only from 0 -> 1). This would mean a couple extra constraints but they're fairly low degree, and we would avoid having any extra degrees in the selector flag.

#### Make the selector columns update "one cycle early":

An idea from Bobbin that hasn't been discussed or investigated in depth yet.
