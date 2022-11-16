
## std::sys
| Procedure | Description |
| ----------- | ------------- |
| truncate_stack | Removes elements deep in the stack until the depth of the stack is exactly 16. The elements<br /><br />are removed in such a way that the top 16 elements of the stack remain unchanged. If the stack<br /><br />would otherwise contain more than 16 elements at the end of execution, then adding a call to this<br /><br />function at the end will reduce the size of the public inputs that are shared with the verifier.<br /><br />Input: Stack with 16 or more elements.<br /><br />Output: Stack with only the original top 16 elements. |
