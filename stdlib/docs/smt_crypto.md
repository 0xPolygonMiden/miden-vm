
## std::crypto::smt
| Procedure | Description |
| ----------- | ------------- |
| get |  Push the leaf value at K to the stack<br /><br /> vars:<br /> R: trie root<br /> K: trie key<br /> V: leaf value<br /><br /> in:  [R, K, ...]<br /> out: [V, R, K, ...] |
| update |  Update the leaf value at K and push the new root R' to the stack <br /><br /> vars:<br /> R: trie root<br /> K: trie key<br /> V: new leaf value<br /><br /> in:  [R, K, V ...]<br /> out: [R', V, K, ...] |
| insert |  Set the leaf value at K and push the new root R' to the stack <br /><br /> vars:<br /> R: old trie root<br /> R': new trie root<br /> K: new trie key<br /> V: new leaf value<br /><br /> in:  [R, K, V ...]<br /> out: [R', V, K, ...] |
