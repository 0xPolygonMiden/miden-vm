
## std::crypto::stark::ood_frames
| Procedure | Description |
| ----------- | ------------- |
| load_evaluation_frame | Loads OOD evaluation frame, with current and next rows interleaved, into memory. This outputs<br />the hash of the OOD for reseeding the random coin.<br /><br />Input: [...]<br />Output: [OOD_FRAME_HASH, ...]<br />Cycles: 106<br /> |
| load_constraint_evaluations | Loads OOD constraint composition polynomial evaluation columns into memory and reseeds the random<br />coin.<br /><br />Input: [...]<br />Output: [EVAL_HASH, ...]<br />Cycles: 112<br /> |
| compute_Hz | Computes the H(z) evaluation of the constraint composition polynomial at the OOD element z.<br /><br />Input: [...]<br />Output: [res1, res0, ...]<br />Cycles: 118<br /> |
