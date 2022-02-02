package types

import (
	"bytes"
	"encoding/binary"
)

const (
	// ModuleName defines the module name
	ModuleName = "qbank"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qbank"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

const (

	// TODO - Use Prefix byte as 0x01, 0x02

	DepositKey                = "Deposit-value-"
	DepositCountKey           = "Deposit-count-"
	UserDenomDepositKeyPrefix = "User-denom-deposit-"
	WithdrawKey               = "Withdraw-value-"
	WithdrawCountKey          = "Withdraw-count-"
)

var (
	// KBP - short of KeyBytePrefix, Byte prfix for the key used in KV store
	QbankGlobalKBP      = []byte{0x00} // Used for counts of deposit and withdraw
	DepositKBP          = []byte{0x01}
	UserDenomDepositKBP = []byte{0x02}
	WithdrawKeyKBP      = []byte{0x03}
	UserDepositKBP      = []byte{0x04}
)

// Common functions for deposit and withdraw

// store key use the byte as key
func createStoreKey(k string) []byte {
	return []byte(k)
}

// create the prefix store key for specific deposit or withdraw object id
// Input param - deposit id or withdraw id
func CreateIDKey(id uint64) []byte {
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, id)
	return bz
}

// create the deposit or withdraw id of uint type from input byte
func CreateIDFromByteKey(bzKey []byte) uint64 {
	return binary.BigEndian.Uint64(bzKey)
}

// Deposit specific function

// create the prefix store key for deposit counts
func CreateDepositCountKey() []byte {
	return createStoreKey(DepositCountKey)
}

// create the prefix store key for the user denom wise deposit storage
func CreateUserDenomDepositKey(uid, sep, denom string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	return b.Bytes()
}

// create the prefix store key for the user denom wise deposit storage
// Ex. {uid} + "/" + {denom} + "/" + "lockupString"
func CreateUserDenomLockupDepositKey(uid, sep, denom string, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

// create the prefix store key for the user denom wise deposit storage
// Ex. {uid} + "/" + {denom} + "/" + {epochday} / "lockupString"
func CreateUserDenomEpochLockupDepositKey(uid, sep, denom string, epochday uint64, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	b.WriteString(sep)
	b.Write(CreateIDKey(epochday))
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

// create the prefix store key for the user denom wise deposit storage
func CreateUserDepositKey(uid string) []byte {
	// var b bytes.Buffer
	// b.WriteString(uid)
	// b.WriteString(sep)
	// b.WriteString(denom)
	// return b.Bytes()
	return createStoreKey(uid)
}

// Withdraw specific functions

// set of key creation functions for withdraw objects
func CreateWithdrawCountKey() []byte {
	return createStoreKey(WithdrawCountKey)
}

const (
	FeeDataKey = "FeeData-value-"
)
