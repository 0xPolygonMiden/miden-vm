
## std::crypto::elgamal_ecgfp5
| Procedure | Description |
| ----------- | ------------- |
| gen_privatekey | Generates the public key, point H<br />the private key is expected as input and is a 319-bit random<br />number of 10 32-bit limbs.<br /> |
| encrypt_ca | Given a random scalar r on stack<br />this routine computes the first elliptic curve point C_a<br />C_a = r*G, G is the generator of elliptic curve<br />Expected stack state<br />[r0, r1, ..., r9]<br />Final stack state<br />[Ca_x0, ..., C_x4, Ca_y0, ..., Ca_y4, Ca_inf]<br /> |
| encrypt_cb | Given public key, point H generated in gen_privatekey as coordinates (X,Y) on stack<br />and message M, elliptic curve points (a,b) also as coordinates (X,Y) on stack<br />and random scalar r on stack<br />this routine computes the second elliptic curve point C_b<br />C_b = M + r*H<br />Expected stack state<br />[H_x0, ..., H_x4, H_y0, ..., H_y4, H_inf, r0, r1, ..., M_x0, ..., M_x4, M_y0, ..., M_y4, M_inf,]<br />Final stack state<br />[Cb_x0, ..., Cb_x4, Cb_y0, ..., Cb_y4, Cb_inf]<br /> |
| remask_ca | Rerandomises the first half of an ElGamal ciphertext Ca<br />and random scalar r to produce a rerandomised ciphertext C'a<br />Expected stack state<br />[r0, r1, ..., Ca_x0, ..., Ca_x4, Ca_y0, ..., Ca_y4, Ca_inf, ...]<br /><br />Final stack state<br />[C'a_x0, ..., C'a_x4, C'a_y0, ..., C'a_y4, C'a_inf]<br /> |
| remask_cb | Rerandomises the second half of an ElGamal ciphertext Cb given a public key H<br />and random scalar r to produce a rerandomised ciphertext C'b<br />Expected stack state<br />[H_x0, ..., H_x4, H_y0, ..., H_y4, H_inf, ..., r0, r1, ..., Cb_x0, ..., Cb_x4, Cb_y0, ..., Cb_y4, Cb_inf]<br /><br />Final stack state<br />[C'b_x0, ..., C'b_x4, C'b_y0, ..., C'b_y4, C'b_inf]<br /> |
