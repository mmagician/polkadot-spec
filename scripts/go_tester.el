;;scale
(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester scale-codec encode --input 1")

;;trie-root
(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester state-trie trie-root --state-file \"../../../../../test/fixtures/random_state_trie_80.yaml\"")

(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester state-trie trie-root --state-file \"../../../../../test/fixtures/1c1_trie.yaml\"")

(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester state-trie trie-root --state-file \"../../../../../test/fixtures/pk_branch_test.yaml\"")

(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester state-trie trie-root --keys-in-hex --state-file \"/home/klaymen/doc/code/polkadot-re-tests/test/fixtures/hex_limit_trie.yaml\"")

;;insert-and-delete
(gdb "gdb -i=mi --cd ../build/bin/usr/local/bin/  --args ./go_tester state-trie insert-and-delete --keys-in-hex --state-file \"/home/klaymen/doc/code/polkadot-re-tests/test/fixtures/hex_limit_trie.yaml\"")


