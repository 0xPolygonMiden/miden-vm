
## std::crypto::fri::frif4e2
| Procedure | Description |
| ----------- | ------------- |
| preprocess | Stores the layer commitments C followed by [d_size, t_depth, a1, a0] where:<br /><br />1) d_size is the domain size divided by 4 of the domain corresponding to C.<br /><br />2) t_depth is the tree depth of the Merkle tree with commitment C.<br /><br />3) (a0, a1) is the folding challenge to create the next layer.<br /><br />TODO: This processing function should in fact compute d_size and t_depth for each C<br /><br />starting from the original domain size. |
| verify_remainder_query | This functions checks that the evaluation at (g^p)^2^n of the last folded codeword is equal<br /><br />to the remainder codeword sent by the prover at (g^p)^2^n.<br /><br />Input: [ptr2n, ptr2n, (g^p)^4, p % d_size, ne1, ne0, ptr2n, ptr2n, x, x, x, x, x, x, x, x, ..]<br /><br />Output: # [ptr2n, ptr2n, x, x, x, x, x, x, x, x, ..]<br /><br />Cost: 20 cycles |
| verify_query_layer | Checks that, for a query with index p at layer i, the folding procedure to create layer (i + 1)<br /><br />was performed correctly.<br /><br />Input: [ptr0 + 2 * i, ptr0 + 2 * i, (g^p)^4, p % d_size, ne1, ne0, ptr0 + 2 * i, ptr2n, x, x, x, x, x, x, x, x, ...]<br /><br />Output: [ptr0 + 2 * i, ptr0 + 2 * i, (g^p)^4, p % d_size, ne1, ne0, ptr0 + 2 * i, ptr2n, x, x, x, x, x, x, x, x, ...]<br /><br />Cost: 83 |
| verify_query | Verifies one FRI query.<br /><br />Input: [g^p, p, e1, e0, ptr0, ptr2n, ..]<br /><br />Output: ()<br /><br />Cost: 37 + num_layers * 105 |
| verify_fri | Verifies a FRI proof<br /><br />Cost: 2626 + 8 + num_queries * (37 + num_layers * 105 + 58) |
