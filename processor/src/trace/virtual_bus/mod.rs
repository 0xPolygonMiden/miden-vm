//! Represents a global virtual bus for the Miden VM.
//!
//! A global bus is a single bus which encompasses several sub-buses, each representing
//! a communication channel between two or more components of the VM.
//! A bus represents a client-server relationship between some VM components. The server is
//! usually one specific component of the VM, e.g., hasher chiplet, and the client can be one or
//! several other components of the VM, e.g., the decoder. The communication between the clients
//! and the server is composed of `request` messages made by the clients and corresponding
//! `reply` messages by the server.
//! The purpose of the sub-bus then, from the verifiable computation point of view, is to ensure
//! the consistency between the `request` and `reply` messages exchanged by the clients and
//! the server.
//! The global bus uses a per-sub-bus address in order to ensure correct routing of the `request`
//! messages and their matching `reply` messages.
//! Miden VM uses a virtual global bus in the sense that neither the global bus nor the individual
//! sub-buses are fully materialized as part of the (auxiliary) trace. This is replaced by a layered
//! circuit which computes the global bus relation. The correct evaluation of this circuit is then
//! proved using the GKR protocol of Goldwasser, Kalai and Rothblum [1] using the protocol in
//! GKR-LogUp [2].
//!
//! [1]: https://dl.acm.org/doi/10.1145/2699436
//! [2]: https://eprint.iacr.org/2023/1284

mod circuit;
pub use circuit::{prove, verify};

mod multilinear;

mod sum_check;
pub use sum_check::{SumCheckProver, SumCheckVerifier};

mod univariate;

#[cfg(test)]
mod tests;
