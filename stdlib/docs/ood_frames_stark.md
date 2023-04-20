
## std::crypto::stark::ood_frames
| Procedure | Description |
| ----------- | ------------- |
| load_ood_eval_frame | Loads and computes ROW_HASH which is the hash of either<br /><br />[ood_main_trace_frame.current(), aux_trace_frame.current()]<br /><br />or [ood_main_trace_frame.next(), aux_trace_frame.next()]<br /><br />Input: [ood_frame_ptr, ...]<br /><br />Output: [ROW_HASH, ...]<br /><br />Cycles: 102 |
| load_evaluation_frame | Loads OOD evaluation frame (both current and next rows) into memory and reseeds the random coin.<br /><br />Input: [...]<br /><br />Output: [CURENT_ROW_HASH, NEXT_ROW_HASH, ...]<br /><br />Cycles: 207 |
| load_constraint_evaluations | Loads OOD constraint composition polynomial evaluation columns into memory and reseeds the random<br /><br />coin.<br /><br />Input: [...]<br /><br />Output: [EVAL_HASH, ...]<br /><br />Cycles: 29 |
| compute_Hz | Computes the H(z) evaluation of the constraint composition polynomial at the OOD element z.<br /><br />Input: [...]<br /><br />Output: [res1, res0, ...]<br /><br />Cycles: 118 |
