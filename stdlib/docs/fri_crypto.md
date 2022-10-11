
## std::crypto::fri
| Procedure | Description |
| ----------- | ------------- |
| fold_2 |  Given a stack in the following initial configuration [a1,a0,b1,b0,c1,c0,d1,d0,...] the following<br /> procedure computes (a + b + ((a - b) * c * d^(-1)))/2 with the assumption that d1 is equal to 0 |
| next_pos_exp |  This procedure computes the folded position in the exponent of the corresponding domain generator<br /> normalized by the offset. It uses an algebraic relationship between the original and folded positions<br /> given by multiplication with the 2nd primitive root of unity in our field.<br /> input:    #[?,poe,poe,...]<br /> output:   #[poe_sq,xs,...] |
| preprocess.1 |  Preprocess the commitments C as well as num_q (number of queries), d (initial domain size),<br /> g (intial domain generator) and t_d (initial tree depth). The address of the word (num_q,d,g,t_d)<br /> will be at locaddr.0. The commitments will be at the subsequent addresses. The total <br /> number of such commitments is t_d - 3 (excluding the remainder). |
| prepare_next |  Input: [t_d,e1,e0,p,d,poe,add',..]<br /> Output: [d,p,C,t_d,e1,e0,poe,a1,a0,add'-2,..] |
| verify_query_layer |  Given a stack in the following initial configuration [d,p,C,t_d,e1,e0,poe,a1,a0,..] the following<br /> procedure computes an iteration of FRI verification for a query.<br /> TODO: Check where/if some checks are needed in the beginning, like p < d (p&d are u32) and that 2^t_d == d  |
| verify_query |  Verify a single Fri verification query. The expected stack configuration is [add,num_q,d,g,t_d,add',...]<br /> where add is the address storing the current (num_q,d,g,t_d), num_q is the index of the current query,<br /> d is the domain size of the LDE, g is its generator and add' is the address of the commitment to <br /> the first layer with the commitments to subsequent layers laid out, in alternating order with the layer<br /> alphas, in the subsequent addresses i.e. add' + 1, add' + 2 ... |
| verify_remainder_com |  Verify that the hash of the remainder codeword is equal to the commitment provided by the prover<br /> The following implementation relies on the assumption that the blowup factor is 8 and that the degree<br /> of the remainder is zero. |
| verify |  |
| outer |  Prepares the memory by populating it with the layer commitments as well as other relevant parameters<br /> needed in order to verify the FRI proof. It then calls ver |
