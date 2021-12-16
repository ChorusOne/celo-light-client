package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/celo-org/celo-blockchain/common/hexutil"
	"github.com/celo-org/celo-blockchain/consensus/istanbul"
	istanbulCore "github.com/celo-org/celo-blockchain/consensus/istanbul/core"
	"github.com/celo-org/celo-blockchain/core/types"
	blscrypto "github.com/celo-org/celo-blockchain/crypto/bls"
	"github.com/celo-org/celo-bls-go/bls"
)

var baklava_file = "data/baklava.json"

func main() {
	bls.InitBLSCrypto()

	jsonFile, err := os.Open(baklava_file)
	if err != nil {
		fmt.Println("os.Open", err)
		return
	}
	defer jsonFile.Close()

	byteValue, err := io.ReadAll(jsonFile)
	if err != nil {
		fmt.Println("io.ReadAll", err)
		return
	}

	headers := make([]types.Header, 100)
	err = json.Unmarshal(byteValue, &headers)
	if err != nil {
		fmt.Println("json.Unmarshal", err)
		return
	}

	validators, err := getGenesisValidators(&headers[0])
	if err != nil {
		fmt.Println("getGenesisValidators", err)
		return
	}

	headers = headers[1:]
	for idx, header := range headers {
		extra, err := types.ExtractIstanbulExtra(&header)
		if err != nil {
			fmt.Println(idx, "types.ExtractIstanbulExtra", err)
			return
		}
		proposalSeal := istanbulCore.PrepareCommittedSeal(header.Hash(), extra.AggregatedSeal.Round)
		publicKeys := []blscrypto.SerializedPublicKey{}
		for i := 0; i < len(validators); i++ {
			if extra.AggregatedSeal.Bitmap.Bit(i) == 1 {
				pubKey := validators[i].BLSPublicKey
				publicKeys = append(publicKeys, pubKey)
			}
		}
		hash := header.Hash()
		fmt.Println(header.Number)
		fmt.Println(proposalSeal)
		fmt.Println(hexutil.Encode(hash[:]))
		fmt.Println(extra.AggregatedSeal.Round)
		return
		err = blscrypto.VerifyAggregatedSignature(publicKeys, proposalSeal, []byte{}, extra.AggregatedSeal.Signature, false, false)
		if err != nil {
			fmt.Println(idx, "blscrypto.VerifyAggregatedSignature", err)
			return
		}
		if idx%10 == 0 {
			fmt.Println("processing header", idx)
		}
	}
}

func getGenesisValidators(head *types.Header) ([]istanbul.ValidatorData, error) {
	extra, err := types.ExtractIstanbulExtra(head)
	if err != nil {
		return nil, err
	}
	validators, err := istanbul.CombineIstanbulExtraToValidatorData(extra.AddedValidators, extra.AddedValidatorsPublicKeys)
	if err != nil {
		return nil, err
	}
	return validators, nil
}
