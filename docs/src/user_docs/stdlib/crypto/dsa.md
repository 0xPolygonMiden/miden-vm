# Digital signatures
Namespace `std::crypto::dsa` contains a set of  digital signature schemes supported by default in the Miden VM. Currently, these schemes are:

* `RPO Falcon512`: a variant of the [Falcon](https://falcon-sign.info/) signature scheme.

## RPO Falcon512

Module `std::crypto::dsa::rpo_falcon512` contains procedures for verifying `RPO Falcon512` signatures. These signatures differ from the standard Falcon signatures in that instead of using `SHAKE256` hash function in the *hash-to-point* algorithm we use `RPO256`. This makes the signature more efficient to verify in the Miden VM.

The module exposes the following procedures:

| Procedure   | Description |
| ----------- | ------------- |
| verify      | Verifies a signature against a public key and a message. The procedure gets as inputs the hash of the public key and the hash of the message via the operand stack. The signature is expected to be provided via the advice provider.<br /><br />The signature is valid if and only if the procedure returns.<br /><br />Inputs: `[PK, MSG, ...]`<br />Outputs: `[...]`<br /><br />Where `PK` is the hash of the public key and `MSG` is the hash of the message. Both hashes are expected to be computed using `RPO` hash function.<br /><br /> The procedure relies on the `adv.push_sig` [decorator](../../assembly/io_operations.md#nondeterministic-inputs) to retrieve the signature from the host. The default host implementation assumes that the private-public key pair is loaded into the advice provider, and uses it to generate the signature. However, for production grade implementations, this functionality should be overridden to ensure more secure handling of private keys.|

