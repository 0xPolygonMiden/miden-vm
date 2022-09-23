
## std::math::gfp5
| Procedure | Description |
| ----------- | ------------- |
| add |  Given two GF(p^5) elements on stack, this routine computes modular<br /> addition over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1<br /><br /> See section 3.2 of https://eprint.iacr.org/2022/274.pdf<br /><br /> For reference implementation in high level language, see <br /> https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L607-L616 |
| sub |  Given two GF(p^5) elements on stack, this routine subtracts second<br /> element from first one, over extension field GF(p^5) s.t. p = 2^64 - 2^32 + 1<br /><br /> See section 3.2 of https://eprint.iacr.org/2022/274.pdf<br /><br /> For reference implementation in high level language, see <br /> https://github.com/pornin/ecgfp5/blob/ce059c6/python/ecGFp5.py#L629-L638 |
