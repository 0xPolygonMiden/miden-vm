# Memory procedures
Module `std::mem` contains a set of utility procedures for working with random access memory.

| Procedure | Description |
| ----------- | ------------- |
| memcopy | Copies `n` words from `read_ptr` to `write_ptr`.<br /><br />Stack transition looks as follows:<br /><br />[n, read_ptr, write_ptr, ...] -> [...]<br /><br />Cycles: 15 + 16n |
