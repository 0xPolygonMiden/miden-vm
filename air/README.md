# Miden VM AIR
This crate contains *algebraic intermediate representation* (AIR) of Miden VM execution logic.

AIR is a STARK-specific format of describing a computation. It consists of defining a set of constraints expressed as low-degree polynomials. Miden VM evaluates these polynomials over an execution trace produced by Miden processor and includes the results in the execution proof. To verify the proof, the verifier checks that the constraints are evaluated correctly against the execution trace committed to by the prover.

Internally, Miden VM AIR is separated into two parts:
* AIR for the decoder, which is responsible for decoding instructions and managing control flow.
* AIR for the stack, which is responsible for executing instructions against the stack.

If you'd like to learn more about AIR, the following blog posts from StarkWare are an excellent resource:

* [Arithmetization I](https://medium.com/starkware/arithmetization-i-15c046390862)
* [Arithmetization II](https://medium.com/starkware/arithmetization-ii-403c3b3f4355)
* [StarkDEX Deep Dive: the STARK Core Engine](https://medium.com/starkware/starkdex-deep-dive-the-stark-core-engine-497942d0f0ab)

## License
This project is [MIT licensed](../LICENSE).