
## std::mem
| Procedure | Description |
| ----------- | ------------- |
| memcopy | Copies `n` words from `read_ptr` to `write_ptr`.<br /><br />Stack transition looks as follows:<br /><br />[n, read_ptr, write_ptr, ...] -> [...]<br /><br />cycles: 15 + 16n |
| pipe_double_words_to_memory | Copies an even number of words from the advice_stack to memory.<br /><br />Input: [C, B, A, write_ptr, end_ptr, ...]<br /><br />Output: [C, B, A, write_ptr, ...]<br /><br />Where:<br /><br />- The words C, B, and A are the RPO hasher state<br /><br />- A is the capacity<br /><br />- C,B are the rate portion of the state<br /><br />- The value `words = end_ptr - write_ptr` must be positive and even<br /><br />Cycles: 10 + 9 * word_pairs |
| pipe_words_to_memory | Copies an arbitrary number of words from the advice stack to memory<br /><br />Input: [num_words, write_ptr, ...]<br /><br />Output: [HASH, write_ptr', ...]<br /><br />Cycles:<br /><br />even num_words: 48 + 9 * num_words / 2<br /><br />odd num_words: 65 + 9 * round_down(num_words / 2) |
| pipe_preimage_to_memory | Moves an arbitrary number of words from the advice stack to memory and asserts it matches the commitment.<br /><br />Input: [num_words, write_ptr, COM, ...]<br /><br />Output: [write_ptr', ...]<br /><br />Cycles:<br /><br />even num_words: 58 + 9 * num_words / 2<br /><br />odd num_words: 75 + 9 * round_down(num_words / 2) |
