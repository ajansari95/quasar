// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: qoracle/coin_rates.proto

package types

import (
	fmt "fmt"
	github_com_cosmos_cosmos_sdk_types "github.com/cosmos/cosmos-sdk/types"
	types "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/gogo/protobuf/proto"
	io "io"
	math "math"
	math_bits "math/bits"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type CoinRatesCallData struct {
	Symbols    []string `protobuf:"bytes,1,rep,name=symbols,proto3" json:"symbols,omitempty"`
	Multiplier uint64   `protobuf:"varint,2,opt,name=multiplier,proto3" json:"multiplier,omitempty"`
}

func (m *CoinRatesCallData) Reset()         { *m = CoinRatesCallData{} }
func (m *CoinRatesCallData) String() string { return proto.CompactTextString(m) }
func (*CoinRatesCallData) ProtoMessage()    {}
func (*CoinRatesCallData) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e7ad660a439b796, []int{0}
}
func (m *CoinRatesCallData) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *CoinRatesCallData) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_CoinRatesCallData.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *CoinRatesCallData) XXX_Merge(src proto.Message) {
	xxx_messageInfo_CoinRatesCallData.Merge(m, src)
}
func (m *CoinRatesCallData) XXX_Size() int {
	return m.Size()
}
func (m *CoinRatesCallData) XXX_DiscardUnknown() {
	xxx_messageInfo_CoinRatesCallData.DiscardUnknown(m)
}

var xxx_messageInfo_CoinRatesCallData proto.InternalMessageInfo

func (m *CoinRatesCallData) GetSymbols() []string {
	if m != nil {
		return m.Symbols
	}
	return nil
}

func (m *CoinRatesCallData) GetMultiplier() uint64 {
	if m != nil {
		return m.Multiplier
	}
	return 0
}

type CoinRatesResult struct {
	Rates []uint64 `protobuf:"varint,1,rep,packed,name=rates,proto3" json:"rates,omitempty"`
}

func (m *CoinRatesResult) Reset()         { *m = CoinRatesResult{} }
func (m *CoinRatesResult) String() string { return proto.CompactTextString(m) }
func (*CoinRatesResult) ProtoMessage()    {}
func (*CoinRatesResult) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e7ad660a439b796, []int{1}
}
func (m *CoinRatesResult) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *CoinRatesResult) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_CoinRatesResult.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *CoinRatesResult) XXX_Merge(src proto.Message) {
	xxx_messageInfo_CoinRatesResult.Merge(m, src)
}
func (m *CoinRatesResult) XXX_Size() int {
	return m.Size()
}
func (m *CoinRatesResult) XXX_DiscardUnknown() {
	xxx_messageInfo_CoinRatesResult.DiscardUnknown(m)
}

var xxx_messageInfo_CoinRatesResult proto.InternalMessageInfo

func (m *CoinRatesResult) GetRates() []uint64 {
	if m != nil {
		return m.Rates
	}
	return nil
}

type CoinRatesLatestRequest struct {
	RequestPacketSequence uint64            `protobuf:"varint,1,opt,name=request_packet_sequence,json=requestPacketSequence,proto3" json:"request_packet_sequence,omitempty"`
	CallData              CoinRatesCallData `protobuf:"bytes,2,opt,name=call_data,json=callData,proto3" json:"call_data"`
	OracleRequestId       uint64            `protobuf:"varint,3,opt,name=oracle_request_id,json=oracleRequestId,proto3" json:"oracle_request_id,omitempty"`
}

func (m *CoinRatesLatestRequest) Reset()         { *m = CoinRatesLatestRequest{} }
func (m *CoinRatesLatestRequest) String() string { return proto.CompactTextString(m) }
func (*CoinRatesLatestRequest) ProtoMessage()    {}
func (*CoinRatesLatestRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e7ad660a439b796, []int{2}
}
func (m *CoinRatesLatestRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *CoinRatesLatestRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_CoinRatesLatestRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *CoinRatesLatestRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_CoinRatesLatestRequest.Merge(m, src)
}
func (m *CoinRatesLatestRequest) XXX_Size() int {
	return m.Size()
}
func (m *CoinRatesLatestRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_CoinRatesLatestRequest.DiscardUnknown(m)
}

var xxx_messageInfo_CoinRatesLatestRequest proto.InternalMessageInfo

func (m *CoinRatesLatestRequest) GetRequestPacketSequence() uint64 {
	if m != nil {
		return m.RequestPacketSequence
	}
	return 0
}

func (m *CoinRatesLatestRequest) GetCallData() CoinRatesCallData {
	if m != nil {
		return m.CallData
	}
	return CoinRatesCallData{}
}

func (m *CoinRatesLatestRequest) GetOracleRequestId() uint64 {
	if m != nil {
		return m.OracleRequestId
	}
	return 0
}

type CoinRatesState struct {
	UpdatedAtHeight int64                                       `protobuf:"varint,2,opt,name=updated_at_height,json=updatedAtHeight,proto3" json:"updated_at_height,omitempty"`
	Rates           github_com_cosmos_cosmos_sdk_types.DecCoins `protobuf:"bytes,4,rep,name=rates,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.DecCoins" json:"rates"`
}

func (m *CoinRatesState) Reset()         { *m = CoinRatesState{} }
func (m *CoinRatesState) String() string { return proto.CompactTextString(m) }
func (*CoinRatesState) ProtoMessage()    {}
func (*CoinRatesState) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e7ad660a439b796, []int{3}
}
func (m *CoinRatesState) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *CoinRatesState) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_CoinRatesState.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *CoinRatesState) XXX_Merge(src proto.Message) {
	xxx_messageInfo_CoinRatesState.Merge(m, src)
}
func (m *CoinRatesState) XXX_Size() int {
	return m.Size()
}
func (m *CoinRatesState) XXX_DiscardUnknown() {
	xxx_messageInfo_CoinRatesState.DiscardUnknown(m)
}

var xxx_messageInfo_CoinRatesState proto.InternalMessageInfo

func (m *CoinRatesState) GetUpdatedAtHeight() int64 {
	if m != nil {
		return m.UpdatedAtHeight
	}
	return 0
}

func (m *CoinRatesState) GetRates() github_com_cosmos_cosmos_sdk_types.DecCoins {
	if m != nil {
		return m.Rates
	}
	return nil
}

func init() {
	proto.RegisterType((*CoinRatesCallData)(nil), "abag.quasarnode.qoracle.CoinRatesCallData")
	proto.RegisterType((*CoinRatesResult)(nil), "abag.quasarnode.qoracle.CoinRatesResult")
	proto.RegisterType((*CoinRatesLatestRequest)(nil), "abag.quasarnode.qoracle.CoinRatesLatestRequest")
	proto.RegisterType((*CoinRatesState)(nil), "abag.quasarnode.qoracle.CoinRatesState")
}

func init() { proto.RegisterFile("qoracle/coin_rates.proto", fileDescriptor_2e7ad660a439b796) }

var fileDescriptor_2e7ad660a439b796 = []byte{
	// 442 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x74, 0x52, 0x4d, 0x6f, 0xd3, 0x40,
	0x10, 0x8d, 0x49, 0xf8, 0xe8, 0x56, 0x22, 0xaa, 0x55, 0xa8, 0x55, 0x21, 0x37, 0xca, 0x85, 0x28,
	0x88, 0x5d, 0xb5, 0x95, 0xb8, 0x93, 0xe6, 0x00, 0x12, 0x95, 0x90, 0x7b, 0xe3, 0x62, 0x8d, 0xd7,
	0x23, 0xc7, 0xea, 0xc6, 0xeb, 0x78, 0xc7, 0x88, 0xfe, 0x0b, 0xfe, 0x04, 0x17, 0x7e, 0x49, 0x25,
	0x2e, 0x3d, 0x72, 0x02, 0x94, 0xfc, 0x11, 0xb4, 0x6b, 0x3b, 0x44, 0x42, 0xbd, 0xd8, 0x3b, 0xfb,
	0xde, 0xbe, 0x79, 0xfb, 0x66, 0x59, 0xb0, 0xd2, 0x15, 0x48, 0x85, 0x42, 0xea, 0xbc, 0x88, 0x2b,
	0x20, 0x34, 0xbc, 0xac, 0x34, 0x69, 0xff, 0x08, 0x12, 0xc8, 0xf8, 0xaa, 0x06, 0x03, 0x55, 0xa1,
	0x53, 0xe4, 0x2d, 0xf3, 0xf8, 0x30, 0xd3, 0x99, 0x76, 0x1c, 0x61, 0x57, 0x0d, 0xfd, 0x38, 0x94,
	0xda, 0x2c, 0xb5, 0x11, 0x09, 0x18, 0x14, 0x9f, 0x4f, 0x13, 0x24, 0x38, 0x75, 0xa2, 0x0d, 0x3e,
	0xbe, 0x64, 0x07, 0x17, 0x3a, 0x2f, 0x22, 0xdb, 0xe1, 0x02, 0x94, 0x9a, 0x03, 0x81, 0x1f, 0xb0,
	0xc7, 0xe6, 0x66, 0x99, 0x68, 0x65, 0x02, 0x6f, 0xd4, 0x9f, 0xec, 0x45, 0x5d, 0xe9, 0x87, 0x8c,
	0x2d, 0x6b, 0x45, 0x79, 0xa9, 0x72, 0xac, 0x82, 0x07, 0x23, 0x6f, 0x32, 0x88, 0x76, 0x76, 0xc6,
	0x2f, 0xd9, 0x70, 0x2b, 0x17, 0xa1, 0xa9, 0x15, 0xf9, 0x87, 0xec, 0xa1, 0xf3, 0xef, 0xa4, 0x06,
	0x51, 0x53, 0x8c, 0x7f, 0x78, 0xec, 0xf9, 0x96, 0xf9, 0xc1, 0x7e, 0x28, 0xc2, 0x55, 0x8d, 0x86,
	0xfc, 0x37, 0xec, 0xa8, 0x6a, 0x96, 0x71, 0x09, 0xf2, 0x1a, 0x29, 0x36, 0xb6, 0x2c, 0x24, 0x06,
	0x9e, 0x6b, 0xf8, 0xac, 0x85, 0x3f, 0x3a, 0xf4, 0xaa, 0x05, 0xfd, 0x4b, 0xb6, 0x27, 0x41, 0xa9,
	0x38, 0x05, 0x02, 0x67, 0x6d, 0xff, 0x6c, 0xca, 0xef, 0x49, 0x8b, 0xff, 0x77, 0xe9, 0xd9, 0xe0,
	0xf6, 0xd7, 0x49, 0x2f, 0x7a, 0x22, 0xbb, 0x10, 0xa6, 0xec, 0xa0, 0xe1, 0xc6, 0x9d, 0x9b, 0x3c,
	0x0d, 0xfa, 0xce, 0xc0, 0xb0, 0x01, 0x5a, 0xc3, 0xef, 0xd3, 0xf1, 0x37, 0x8f, 0x3d, 0xdd, 0x2a,
	0x5e, 0x11, 0x10, 0xda, 0xe3, 0x75, 0x99, 0x02, 0x61, 0x1a, 0x03, 0xc5, 0x0b, 0xcc, 0xb3, 0x05,
	0x39, 0x57, 0xfd, 0x68, 0xd8, 0x02, 0x6f, 0xe9, 0x9d, 0xdb, 0xf6, 0xb3, 0x2e, 0xa2, 0xc1, 0xa8,
	0x3f, 0xd9, 0x3f, 0x7b, 0xc1, 0x9b, 0xa1, 0x71, 0x3b, 0x34, 0xde, 0x0e, 0x8d, 0xcf, 0x51, 0xda,
	0x16, 0xb3, 0x73, 0xeb, 0xf3, 0xfb, 0xef, 0x93, 0x57, 0x59, 0x4e, 0x8b, 0x3a, 0xe1, 0x52, 0x2f,
	0x45, 0x3b, 0xe4, 0xe6, 0xf7, 0xda, 0xa4, 0xd7, 0x82, 0x6e, 0x4a, 0x34, 0xdd, 0x19, 0xd3, 0xa6,
	0x3e, 0x9b, 0xdf, 0xae, 0x43, 0xef, 0x6e, 0x1d, 0x7a, 0x7f, 0xd6, 0xa1, 0xf7, 0x75, 0x13, 0xf6,
	0xee, 0x36, 0x61, 0xef, 0xe7, 0x26, 0xec, 0x7d, 0x9a, 0xee, 0xa8, 0xd9, 0xcc, 0xc4, 0xbf, 0xcc,
	0xc4, 0x17, 0xd1, 0xbd, 0x46, 0xa7, 0x9a, 0x3c, 0x72, 0x4f, 0xe7, 0xfc, 0x6f, 0x00, 0x00, 0x00,
	0xff, 0xff, 0xaa, 0xf4, 0x43, 0xe4, 0xa5, 0x02, 0x00, 0x00,
}

func (m *CoinRatesCallData) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CoinRatesCallData) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *CoinRatesCallData) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.Multiplier != 0 {
		i = encodeVarintCoinRates(dAtA, i, uint64(m.Multiplier))
		i--
		dAtA[i] = 0x10
	}
	if len(m.Symbols) > 0 {
		for iNdEx := len(m.Symbols) - 1; iNdEx >= 0; iNdEx-- {
			i -= len(m.Symbols[iNdEx])
			copy(dAtA[i:], m.Symbols[iNdEx])
			i = encodeVarintCoinRates(dAtA, i, uint64(len(m.Symbols[iNdEx])))
			i--
			dAtA[i] = 0xa
		}
	}
	return len(dAtA) - i, nil
}

func (m *CoinRatesResult) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CoinRatesResult) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *CoinRatesResult) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Rates) > 0 {
		dAtA2 := make([]byte, len(m.Rates)*10)
		var j1 int
		for _, num := range m.Rates {
			for num >= 1<<7 {
				dAtA2[j1] = uint8(uint64(num)&0x7f | 0x80)
				num >>= 7
				j1++
			}
			dAtA2[j1] = uint8(num)
			j1++
		}
		i -= j1
		copy(dAtA[i:], dAtA2[:j1])
		i = encodeVarintCoinRates(dAtA, i, uint64(j1))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *CoinRatesLatestRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CoinRatesLatestRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *CoinRatesLatestRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.OracleRequestId != 0 {
		i = encodeVarintCoinRates(dAtA, i, uint64(m.OracleRequestId))
		i--
		dAtA[i] = 0x18
	}
	{
		size, err := m.CallData.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintCoinRates(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x12
	if m.RequestPacketSequence != 0 {
		i = encodeVarintCoinRates(dAtA, i, uint64(m.RequestPacketSequence))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func (m *CoinRatesState) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CoinRatesState) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *CoinRatesState) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Rates) > 0 {
		for iNdEx := len(m.Rates) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.Rates[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintCoinRates(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x22
		}
	}
	if m.UpdatedAtHeight != 0 {
		i = encodeVarintCoinRates(dAtA, i, uint64(m.UpdatedAtHeight))
		i--
		dAtA[i] = 0x10
	}
	return len(dAtA) - i, nil
}

func encodeVarintCoinRates(dAtA []byte, offset int, v uint64) int {
	offset -= sovCoinRates(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *CoinRatesCallData) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if len(m.Symbols) > 0 {
		for _, s := range m.Symbols {
			l = len(s)
			n += 1 + l + sovCoinRates(uint64(l))
		}
	}
	if m.Multiplier != 0 {
		n += 1 + sovCoinRates(uint64(m.Multiplier))
	}
	return n
}

func (m *CoinRatesResult) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if len(m.Rates) > 0 {
		l = 0
		for _, e := range m.Rates {
			l += sovCoinRates(uint64(e))
		}
		n += 1 + sovCoinRates(uint64(l)) + l
	}
	return n
}

func (m *CoinRatesLatestRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.RequestPacketSequence != 0 {
		n += 1 + sovCoinRates(uint64(m.RequestPacketSequence))
	}
	l = m.CallData.Size()
	n += 1 + l + sovCoinRates(uint64(l))
	if m.OracleRequestId != 0 {
		n += 1 + sovCoinRates(uint64(m.OracleRequestId))
	}
	return n
}

func (m *CoinRatesState) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.UpdatedAtHeight != 0 {
		n += 1 + sovCoinRates(uint64(m.UpdatedAtHeight))
	}
	if len(m.Rates) > 0 {
		for _, e := range m.Rates {
			l = e.Size()
			n += 1 + l + sovCoinRates(uint64(l))
		}
	}
	return n
}

func sovCoinRates(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozCoinRates(x uint64) (n int) {
	return sovCoinRates(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *CoinRatesCallData) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoinRates
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: CoinRatesCallData: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CoinRatesCallData: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Symbols", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthCoinRates
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthCoinRates
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Symbols = append(m.Symbols, string(dAtA[iNdEx:postIndex]))
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Multiplier", wireType)
			}
			m.Multiplier = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Multiplier |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoinRates(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthCoinRates
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *CoinRatesResult) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoinRates
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: CoinRatesResult: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CoinRatesResult: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType == 0 {
				var v uint64
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowCoinRates
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					v |= uint64(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				m.Rates = append(m.Rates, v)
			} else if wireType == 2 {
				var packedLen int
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return ErrIntOverflowCoinRates
					}
					if iNdEx >= l {
						return io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					packedLen |= int(b&0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				if packedLen < 0 {
					return ErrInvalidLengthCoinRates
				}
				postIndex := iNdEx + packedLen
				if postIndex < 0 {
					return ErrInvalidLengthCoinRates
				}
				if postIndex > l {
					return io.ErrUnexpectedEOF
				}
				var elementCount int
				var count int
				for _, integer := range dAtA[iNdEx:postIndex] {
					if integer < 128 {
						count++
					}
				}
				elementCount = count
				if elementCount != 0 && len(m.Rates) == 0 {
					m.Rates = make([]uint64, 0, elementCount)
				}
				for iNdEx < postIndex {
					var v uint64
					for shift := uint(0); ; shift += 7 {
						if shift >= 64 {
							return ErrIntOverflowCoinRates
						}
						if iNdEx >= l {
							return io.ErrUnexpectedEOF
						}
						b := dAtA[iNdEx]
						iNdEx++
						v |= uint64(b&0x7F) << shift
						if b < 0x80 {
							break
						}
					}
					m.Rates = append(m.Rates, v)
				}
			} else {
				return fmt.Errorf("proto: wrong wireType = %d for field Rates", wireType)
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoinRates(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthCoinRates
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *CoinRatesLatestRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoinRates
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: CoinRatesLatestRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CoinRatesLatestRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field RequestPacketSequence", wireType)
			}
			m.RequestPacketSequence = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.RequestPacketSequence |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CallData", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthCoinRates
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthCoinRates
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.CallData.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field OracleRequestId", wireType)
			}
			m.OracleRequestId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.OracleRequestId |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoinRates(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthCoinRates
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *CoinRatesState) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoinRates
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: CoinRatesState: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CoinRatesState: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field UpdatedAtHeight", wireType)
			}
			m.UpdatedAtHeight = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.UpdatedAtHeight |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Rates", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthCoinRates
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthCoinRates
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Rates = append(m.Rates, types.DecCoin{})
			if err := m.Rates[len(m.Rates)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipCoinRates(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthCoinRates
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipCoinRates(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowCoinRates
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowCoinRates
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthCoinRates
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupCoinRates
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthCoinRates
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthCoinRates        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowCoinRates          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupCoinRates = fmt.Errorf("proto: unexpected end of group")
)
