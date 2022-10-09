
## std::math::fri
| Procedure | Description |
| ----------- | ------------- |
| fold_2 |  Given a stack in the following initial configuration [a1,a0,b1,b0,c1,c0,d1,d0,...] the following<br /> procedure computes (a + b + ((a - b) * c * d^(-1))) with the assumption that d1 is equal to 0 |
| next_pos_exp |  This procedure computes the folded position in the exponent of the corresponding domain generator<br /> normalized by the offset. It uses an algebraic relationship between the original and folded positions<br /> given by multiplication with the 2nd primitive root of unity in our field.<br /> input:    #[?,poe,poe,...]<br /> output:   #[poe_sq,xs,...] |
| prepare_next_new |  Input: [t_d,e1,e0,p,d,poe,add',..]<br /> Output: [d,p,C,t_d,e1,e0,poe,a1,a0,..] |
| verify_query_layer |  Given a stack in the following initial configuration [d,p,C,t_d,e1,e0,poe,a1,a0,..] the following<br /> procedure computes an iteration of fri verification for a query.<br /> TODO: Check where/if some checks are needed in the beginning, like p < d (p&d are u32) and that 2^t_d == d  |
| outer.1 |  |
