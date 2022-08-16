
## std::crypto::dsa::falcon
| Procedure | Description |
| ----------- | ------------- |
| normalize_poly512.128 |  Given a degree 512 polynomial on stack, using its absolute memory addresses, this routine<br /> normalizes each coefficient of the polynomial, using above defined `normalize()` routine<br /><br /> Imagine, f is the given polynomial of degree 512. It can be normalized using<br /><br /> g = [normalize(f[i]) for i in range(512)]<br /><br /> Expected stack state :<br /><br /> [f_addr0, f_addr1, ..., f_addr127, ...] \| 128 absolute memory addresses<br /><br /> Post normalization stack state looks like<br /><br /> [g_addr0, g_addr1, ..., g_addr127, ...] \| 128 absolute memory addresses<br /><br /> Note, input polynomial which is provided using memory addresses, is not mutated. |
