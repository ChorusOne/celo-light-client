package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/big"
	"math/rand"
	"strings"

	"github.com/celo-org/celo-blockchain/common"
	"github.com/celo-org/celo-blockchain/common/hexutil"
	"github.com/celo-org/celo-blockchain/core/types"
	blscrypto "github.com/celo-org/celo-blockchain/crypto/bls"
    istanbulcore "github.com/celo-org/celo-blockchain/consensus/istanbul/core"
	"github.com/celo-org/celo-blockchain/rlp"
)

func main() {
	rand.Seed(54)
	Generate_seal_test()
	fmt.Println()
	Generate_extra_test()
	fmt.Println()
	Generate_header_test(1)
	Generate_header_test(2)
	Generate_header_test(3)
	fmt.Println()
	Generate_committed_seal_test()
}

func Generate_extra_test() {
	extra := GenerateIstanbulExtra()
	bb, e := rlp.EncodeToBytes(&extra)
	if e != nil {
		fmt.Println("error in test", e)
		return
	}
	t := hexutil.Encode(bb)
    fmt.Println("test: IstanbulExtra")
	fmt.Println("hex", t[2:])
	fmt.Println(PrintExtra(&extra))
}
func Generate_seal_test() {
	seal := GenerateSeal()
	bb, e := rlp.EncodeToBytes(&seal)
	if e != nil {
		fmt.Println("error in test", e)
		return
	}
	t := hexutil.Encode(bb)
    fmt.Println("test: AggregatedSeal")
	fmt.Println("hex", t[2:])
	fmt.Println(printSeal(&seal))
}
func Generate_header_test(idx int) {
	header := GenerateHeader()
	bb, e := rlp.EncodeToBytes(&header)
	if e != nil {
		fmt.Println("error in test", e)
		return
	}
	bbJson, e := json.Marshal(header)
	if e != nil {
		fmt.Println("error in test", e)
		return
	}
	t := hexutil.Encode(bb)
    fmt.Println("test : Header -", idx)
	fmt.Println("hex", t[2:])
    fmt.Println("json", string(bbJson))
    fmt.Println("hash", hexutil.Encode(header.Hash().Bytes()))
	fmt.Println(PrintHeader(&header))
}

func Generate_committed_seal_test() {
    hash := randomHash()
    round := randomInt()

    bb := istanbulcore.PrepareCommittedSeal(hash, round)
    bb_str := make([]string, len(bb))
    for idx, b := range bb {
        bb_str[idx] = fmt.Sprintf("%d", b)
    }


    fmt.Println("test: PrepareCommittedSeal")
	fmt.Println(`let round = U128::from(`, round, `as u64);`)
	fmt.Printf(`let hash = H256::from_str("%s").unwrap();`, hexutil.Encode(hash.Bytes())[2:])
	fmt.Println()
    fmt.Println(`let expected_seal :Vec<u8> = vec![`, strings.Join(bb_str, ", "), `];`)
}

func GenerateSeal() types.IstanbulAggregatedSeal {
	return types.IstanbulAggregatedSeal{
		Bitmap:    randomInt(),
		Signature: randomBytes(32),
		Round:     randomInt(),
	}

}
func GenerateIstanbulExtra() types.IstanbulExtra {
	extra := types.IstanbulExtra{
		AddedValidators: []common.Address{
			randomAddress(),
			randomAddress(),
			randomAddress(),
			randomAddress(),
		},
		AddedValidatorsPublicKeys: []blscrypto.SerializedPublicKey{
			randomPublicKey(),
			randomPublicKey(),
			randomPublicKey(),
		},
		RemovedValidators:    big.NewInt(rand.Int63()),
		Seal:                 randomBytes(16),
		AggregatedSeal:       GenerateSeal(),
		ParentAggregatedSeal: GenerateSeal(),
	}
	return extra
}
func GenerateHeader() types.Header {
	header := types.Header{
		ParentHash:  randomHash(),
		Coinbase:    randomAddress(),
		Root:        randomHash(),
		TxHash:      randomHash(),
		ReceiptHash: randomHash(),
		Bloom:       randomBloom(),
		Number:      randomInt(),
		GasUsed:     rand.Uint64(),
		Time:        rand.Uint64(),
		Extra:       randomBytes(64),
	}
	return header
}

func randomInt() *big.Int {
	return big.NewInt(rand.Int63())
}
func randomAddress() common.Address {
	return common.BigToAddress(randomInt())
}
func randomHash() common.Hash {
	return common.BigToHash(randomInt())
}
func randomPublicKey() blscrypto.SerializedPublicKey {
	var r blscrypto.SerializedPublicKey
	rand.Read(r[:])
	return r
}
func randomBloom() types.Bloom {
	var b types.Bloom
	rand.Read(b[:])
	return b
}
func randomBytes(n uint) []byte {
	var bb = make([]byte, n)
	rand.Read(bb)
	return bb
}

func printSeal(seal *types.IstanbulAggregatedSeal) string {
	buf := new(bytes.Buffer)
	fmt.Fprintf(buf, `IstanbulAggregatedSeal{`)
	if seal.Bitmap == nil {
		fmt.Fprint(buf, `bitmap: U128::from(0 as u64),`)
	} else {
		fmt.Fprintf(buf, `bitmap: U128::from(%d as u64),`, seal.Bitmap)
	}
	fmt.Fprintf(buf, `signature: hex::decode("%s").unwrap(),`, hexutil.Encode(seal.Signature)[2:])
	if seal.Round == nil {
		fmt.Fprint(buf, `round: U128::from(0 as u64)`)
	} else {
		fmt.Fprintf(buf, `round: U128::from(%d as u64)`, seal.Round)
	}
	fmt.Fprint(buf, "}")
	return buf.String()
}

func PrintExtra(extra *types.IstanbulExtra) string {
	buf := new(bytes.Buffer)
	{
		elems := make([]string, len(extra.AddedValidators))
		fmt.Fprint(buf, "IstanbulExtra{")
		for idx, item := range extra.AddedValidators {
			elems[idx] = fmt.Sprint(`H160::from_str("`, hexutil.Encode(item.Bytes())[2:], `").unwrap()`)
		}
		fmt.Fprintf(buf, "added_validators: vec![%s], ", strings.Join(elems, ", "))
	}
	{
		elems := make([]string, len(extra.AddedValidatorsPublicKeys))
		for idx, item := range extra.AddedValidatorsPublicKeys {
			elems[idx] = fmt.Sprint(`SerializedPublicKey::from_str("`, hexutil.Encode(item[:])[2:], `").unwrap()`)
		}
		fmt.Fprintf(buf, "added_validators_public_keys: vec![%s], ", strings.Join(elems, ", "))
	}
	if extra.RemovedValidators == nil {
		fmt.Fprint(buf, "removed_validators: U128::from(0 as u64), ")
	} else {
		fmt.Fprintf(buf, "removed_validators: U128::from(%d as u64), ", extra.RemovedValidators)
	}
	fmt.Fprintf(buf, `seal: hex::decode("%s").unwrap(), `, hexutil.Encode(extra.Seal)[2:])
	fmt.Fprintf(buf, `aggregated_seal: %s,`, printSeal(&extra.AggregatedSeal))
	fmt.Fprintf(buf, `parent_aggregated_seal: %s`, printSeal(&extra.ParentAggregatedSeal))
	fmt.Fprint(buf, "}")
	return buf.String()
}
func PrintHeader(header *types.Header) string {
	buf := new(bytes.Buffer)
	fmt.Fprint(buf, "Header{")
	fmt.Fprint(buf, `parent_hash: H256::from_str("`, hexutil.Encode(header.ParentHash.Bytes())[2:], `").unwrap(), `)
	fmt.Fprint(buf, `coinbase: H160::from_str("`, hexutil.Encode(header.Coinbase.Bytes())[2:], `").unwrap(), `)
	fmt.Fprint(buf, `root: H256::from_str("`, hexutil.Encode(header.Root.Bytes())[2:], `").unwrap(), `)
	fmt.Fprint(buf, `tx_hash: H256::from_str("`, hexutil.Encode(header.TxHash.Bytes())[2:], `").unwrap(), `)
	fmt.Fprint(buf, `receipt_hash: H256::from_str("`, hexutil.Encode(header.ReceiptHash.Bytes())[2:], `").unwrap(), `)
	fmt.Fprint(buf, `bloom: Bloom::from_str("`, hexutil.Encode(header.Bloom.Bytes())[2:], `").unwrap(), `)
    fmt.Fprint(buf, `number: U64::from(`, header.Number, ` as u64), `)
	fmt.Fprint(buf, `gas_used: U256::from(`, header.GasUsed, ` as u64), `)
	fmt.Fprint(buf, `time: U256::from(`, header.Time, ` as u64), `)
	fmt.Fprintf(buf, `extra: hex::decode("%s").unwrap()`, hexutil.Encode(header.Extra)[2:])
	fmt.Fprint(buf, "}")
	return buf.String()
}
