
## std::math::ext2
| Procedure | Description |
| ----------- | ------------- |
| mul | Given a stack with initial configuration given by [a1,a0,b1,b0,...] where a = (a0,a1) and<br /><br />b = (b0,b1) represent elements in the extension field of degree 2, the procedure outputs the<br /><br />product c = (c1,c0) where c0 = a0b0 - 2(a1b1) and c1 = (a0 + a1)(b0 + b1) - a0b0 |
| mul_base | Given a stack with initial configuration given by [x,a1,a0,...] where a = (a0,a1) is an element<br /><br />in the field extension and x is an element of the base field, this procedure computes the multiplication<br /><br />of x, when looked at as (x,0), with a in the extension field. The output is [xa1,xa0,...] |
| add | Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following<br /><br />procedure computes [a1+b1,a0+b0,...] |
| sub | Given a stack in the following initial configuration [a1,a0,b1,b0,...] the following<br /><br />procedure computes [a1-b1,a0-b0,...] |
