package main

import (
	"bytes"
	crand "crypto/rand"
	"encoding/json"
	"fmt"
	"log"

	"github.com/celo-org/celo-blockchain/common"
	"github.com/celo-org/celo-blockchain/crypto"
	"github.com/celo-org/celo-blockchain/ethdb/memorydb"
	"github.com/celo-org/celo-blockchain/trie"
)

func main() {
	tree, vals := randomTrie(500)
	bb, err := json.Marshal(tree)
	if err != nil {
		log.Fatal("json.Marshal(tree)", err)
	}
	fmt.Println(string(bb))
	root := tree.Hash()
	for i, prover := range makeProvers(tree) {
		for j, kv := range vals {
			proof := prover(kv.k)
			bb, err := json.Marshal(proof)
			if err != nil {
				log.Fatal("json.Marshal(proof)", err)
			}
			fmt.Println(i, j,bb)
			if proof == nil {
				log.Fatalf("prover %d: missing key %x while constructing proof", i, kv.k)
			}
			val, err := trie.VerifyProof(root, kv.k, proof)
			if err != nil {
				log.Fatalf("prover %d: failed to verify proof for key %x: %v\nraw proof: %x", i, kv.k, err, proof)
			}
			if !bytes.Equal(val, kv.v) {
				log.Fatalf("prover %d: verified value mismatch for key %x: have %x, want %x", i, kv.k, val, kv.v)
			}
		}
	}
}

// copied from https://github.com/celo-org/celo-blockchain/blob/master/trie/iterator_test.go#L60
type kv struct {
	k, v []byte
	t    bool
}

// copied from https://github.com/celo-org/celo-blockchain/blob/master/trie/proof_test.go#L887
func randBytes(n int) []byte {
	r := make([]byte, n)
	crand.Read(r)
	return r
}

// copied from https://github.com/celo-org/celo-blockchain/blob/master/trie/proof_test.go#L868
func randomTrie(n int) (*trie.Trie, map[string]*kv) {
	trie := new(trie.Trie)
	vals := make(map[string]*kv)
	for i := byte(0); i < 100; i++ {
		value := &kv{common.LeftPadBytes([]byte{i}, 32), []byte{i}, false}
		value2 := &kv{common.LeftPadBytes([]byte{i + 10}, 32), []byte{i}, false}
		trie.Update(value.k, value.v)
		trie.Update(value2.k, value2.v)
		vals[string(value.k)] = value
		vals[string(value2.k)] = value2
	}
	for i := 0; i < n; i++ {
		value := &kv{randBytes(32), randBytes(20), false}
		trie.Update(value.k, value.v)
		vals[string(value.k)] = value
	}
	return trie, vals
}

// copied from https://github.com/celo-org/celo-blockchain/blob/master/trie/proof_test.go#L38
func makeProvers(tree *trie.Trie) []func(key []byte) *memorydb.Database {
	var provers []func(key []byte) *memorydb.Database

	// Create a direct trie based Merkle prover
	provers = append(provers, func(key []byte) *memorydb.Database {
		proof := memorydb.New()
		tree.Prove(key, 0, proof)
		return proof
	})
	// Create a leaf iterator based Merkle prover
	provers = append(provers, func(key []byte) *memorydb.Database {
		proof := memorydb.New()
		if it := trie.NewIterator(tree.NodeIterator(key)); it.Next() && bytes.Equal(key, it.Key) {
			for _, p := range it.Prove() {
				proof.Put(crypto.Keccak256(p), p)
			}
		}
		return proof
	})
	return provers
}
