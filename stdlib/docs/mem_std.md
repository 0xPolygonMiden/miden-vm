
## std::mem
| Procedure | Description |
| ----------- | ------------- |
| memcopy | Copies `n` words from `read_ptr` to `write_ptr`.<br /><br />Stack transition looks as follows:<br /><br />[n, read_ptr, write_ptr, ...] -> [...]<br /><br />cycles: 15 + 16n |
