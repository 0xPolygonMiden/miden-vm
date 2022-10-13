
## std::math::ec_ext5
| Procedure | Description |
| ----------- | ------------- |
| validate |  Given an encoded elliptic curve point `w` s.t. it's expressed using<br /> an element ∈ GF(p^5) \| p = 2^64 - 2^32 + 1, this routine verifies whether<br /> given point can be successfully decoded or not<br /><br /> Expected stack state <br /><br /> [w0, w1, w2, w3, w4, ...]<br /><br /> Final stack state <br /><br /> [flg, ...]<br /><br /> If w can be decoded, flg = 1<br /> Else flg = 0<br /><br /> Note, if w = (0, 0, 0, 0, 0), it can be successfully decoded to point <br /> at infinity i.e. flg = 1, in that case.<br /><br /> See https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L1043-L1052<br /> for reference implementation |
