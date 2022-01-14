// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: qbank/fee_data.proto

package types

import (
	fmt "fmt"
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

type FeeData struct {
	FeeCollector string     `protobuf:"bytes,1,opt,name=feeCollector,proto3" json:"feeCollector,omitempty"`
	FromAddress  string     `protobuf:"bytes,2,opt,name=fromAddress,proto3" json:"fromAddress,omitempty"`
	Fee          types.Coin `protobuf:"bytes,3,opt,name=fee,proto3" json:"fee"`
	FeeType      uint64     `protobuf:"varint,4,opt,name=feeType,proto3" json:"feeType,omitempty"`
	BlockHeight  uint64     `protobuf:"varint,5,opt,name=blockHeight,proto3" json:"blockHeight,omitempty"`
	Memo         string     `protobuf:"bytes,6,opt,name=memo,proto3" json:"memo,omitempty"`
}

func (m *FeeData) Reset()         { *m = FeeData{} }
func (m *FeeData) String() string { return proto.CompactTextString(m) }
func (*FeeData) ProtoMessage()    {}
func (*FeeData) Descriptor() ([]byte, []int) {
	return fileDescriptor_f74593b050daddd8, []int{0}
}
func (m *FeeData) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *FeeData) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_FeeData.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *FeeData) XXX_Merge(src proto.Message) {
	xxx_messageInfo_FeeData.Merge(m, src)
}
func (m *FeeData) XXX_Size() int {
	return m.Size()
}
func (m *FeeData) XXX_DiscardUnknown() {
	xxx_messageInfo_FeeData.DiscardUnknown(m)
}

var xxx_messageInfo_FeeData proto.InternalMessageInfo

func (m *FeeData) GetFeeCollector() string {
	if m != nil {
		return m.FeeCollector
	}
	return ""
}

func (m *FeeData) GetFromAddress() string {
	if m != nil {
		return m.FromAddress
	}
	return ""
}

func (m *FeeData) GetFee() types.Coin {
	if m != nil {
		return m.Fee
	}
	return types.Coin{}
}

func (m *FeeData) GetFeeType() uint64 {
	if m != nil {
		return m.FeeType
	}
	return 0
}

func (m *FeeData) GetBlockHeight() uint64 {
	if m != nil {
		return m.BlockHeight
	}
	return 0
}

func (m *FeeData) GetMemo() string {
	if m != nil {
		return m.Memo
	}
	return ""
}

func init() {
	proto.RegisterType((*FeeData)(nil), "abag.quasarnode.qbank.FeeData")
}

func init() { proto.RegisterFile("qbank/fee_data.proto", fileDescriptor_f74593b050daddd8) }

var fileDescriptor_f74593b050daddd8 = []byte{
	// 303 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x54, 0x90, 0xb1, 0x4e, 0xeb, 0x30,
	0x14, 0x86, 0xe3, 0xdb, 0xdc, 0x56, 0xd7, 0xbd, 0x93, 0x55, 0x24, 0xd3, 0xc1, 0x44, 0x9d, 0x32,
	0xd9, 0x2a, 0x3c, 0x01, 0x2d, 0x42, 0xcc, 0x15, 0x13, 0x0b, 0xb2, 0x93, 0x93, 0x34, 0x6a, 0x93,
	0xd3, 0xc6, 0x2e, 0xa2, 0x6f, 0xc1, 0x63, 0x75, 0xac, 0x98, 0x98, 0x10, 0x6a, 0x5f, 0x04, 0xc5,
	0x01, 0x51, 0xb6, 0xe3, 0xff, 0xff, 0xa4, 0xdf, 0xfa, 0xe8, 0x60, 0x6d, 0x74, 0xb5, 0x50, 0x19,
	0xc0, 0x63, 0xaa, 0x9d, 0x96, 0xab, 0x1a, 0x1d, 0xb2, 0x33, 0x6d, 0x74, 0x2e, 0xd7, 0x1b, 0x6d,
	0x75, 0x5d, 0x61, 0x0a, 0xd2, 0x53, 0xc3, 0x41, 0x8e, 0x39, 0x7a, 0x42, 0x35, 0x57, 0x0b, 0x0f,
	0x45, 0x82, 0xb6, 0x44, 0xab, 0x8c, 0xb6, 0xa0, 0x9e, 0xc6, 0x06, 0x9c, 0x1e, 0xab, 0x04, 0x8b,
	0xaa, 0xed, 0x47, 0xaf, 0x84, 0xf6, 0x6e, 0x01, 0x6e, 0xb4, 0xd3, 0x6c, 0x44, 0xff, 0x67, 0x00,
	0x53, 0x5c, 0x2e, 0x21, 0x71, 0x58, 0x73, 0x12, 0x91, 0xf8, 0xdf, 0xec, 0x57, 0xc6, 0x22, 0xda,
	0xcf, 0x6a, 0x2c, 0xaf, 0xd3, 0xb4, 0x06, 0x6b, 0xf9, 0x1f, 0x8f, 0x9c, 0x46, 0x6c, 0x4c, 0x3b,
	0x19, 0x00, 0xef, 0x44, 0x24, 0xee, 0x5f, 0x9e, 0xcb, 0x76, 0x5f, 0x36, 0xfb, 0xf2, 0x6b, 0x5f,
	0x4e, 0xb1, 0xa8, 0x26, 0xe1, 0xee, 0xfd, 0x22, 0x98, 0x35, 0x2c, 0xe3, 0xb4, 0x97, 0x01, 0xdc,
	0x6f, 0x57, 0xc0, 0xc3, 0x88, 0xc4, 0xe1, 0xec, 0xfb, 0xd9, 0xcc, 0x99, 0x25, 0x26, 0x8b, 0x3b,
	0x28, 0xf2, 0xb9, 0xe3, 0x7f, 0x7d, 0x7b, 0x1a, 0x31, 0x46, 0xc3, 0x12, 0x4a, 0xe4, 0x5d, 0xff,
	0x13, 0x7f, 0x4f, 0x26, 0xbb, 0x83, 0x20, 0xfb, 0x83, 0x20, 0x1f, 0x07, 0x41, 0x5e, 0x8e, 0x22,
	0xd8, 0x1f, 0x45, 0xf0, 0x76, 0x14, 0xc1, 0x43, 0x9c, 0x17, 0x6e, 0xbe, 0x31, 0x32, 0xc1, 0x52,
	0x35, 0x1a, 0xd5, 0x8f, 0x46, 0xf5, 0xac, 0x5a, 0xdd, 0x6e, 0xbb, 0x02, 0x6b, 0xba, 0xde, 0xcf,
	0xd5, 0x67, 0x00, 0x00, 0x00, 0xff, 0xff, 0x70, 0x92, 0xd1, 0xb1, 0x84, 0x01, 0x00, 0x00,
}

func (m *FeeData) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *FeeData) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *FeeData) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Memo) > 0 {
		i -= len(m.Memo)
		copy(dAtA[i:], m.Memo)
		i = encodeVarintFeeData(dAtA, i, uint64(len(m.Memo)))
		i--
		dAtA[i] = 0x32
	}
	if m.BlockHeight != 0 {
		i = encodeVarintFeeData(dAtA, i, uint64(m.BlockHeight))
		i--
		dAtA[i] = 0x28
	}
	if m.FeeType != 0 {
		i = encodeVarintFeeData(dAtA, i, uint64(m.FeeType))
		i--
		dAtA[i] = 0x20
	}
	{
		size, err := m.Fee.MarshalToSizedBuffer(dAtA[:i])
		if err != nil {
			return 0, err
		}
		i -= size
		i = encodeVarintFeeData(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0x1a
	if len(m.FromAddress) > 0 {
		i -= len(m.FromAddress)
		copy(dAtA[i:], m.FromAddress)
		i = encodeVarintFeeData(dAtA, i, uint64(len(m.FromAddress)))
		i--
		dAtA[i] = 0x12
	}
	if len(m.FeeCollector) > 0 {
		i -= len(m.FeeCollector)
		copy(dAtA[i:], m.FeeCollector)
		i = encodeVarintFeeData(dAtA, i, uint64(len(m.FeeCollector)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func encodeVarintFeeData(dAtA []byte, offset int, v uint64) int {
	offset -= sovFeeData(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *FeeData) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.FeeCollector)
	if l > 0 {
		n += 1 + l + sovFeeData(uint64(l))
	}
	l = len(m.FromAddress)
	if l > 0 {
		n += 1 + l + sovFeeData(uint64(l))
	}
	l = m.Fee.Size()
	n += 1 + l + sovFeeData(uint64(l))
	if m.FeeType != 0 {
		n += 1 + sovFeeData(uint64(m.FeeType))
	}
	if m.BlockHeight != 0 {
		n += 1 + sovFeeData(uint64(m.BlockHeight))
	}
	l = len(m.Memo)
	if l > 0 {
		n += 1 + l + sovFeeData(uint64(l))
	}
	return n
}

func sovFeeData(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozFeeData(x uint64) (n int) {
	return sovFeeData(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *FeeData) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowFeeData
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
			return fmt.Errorf("proto: FeeData: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: FeeData: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field FeeCollector", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
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
				return ErrInvalidLengthFeeData
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthFeeData
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.FeeCollector = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field FromAddress", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
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
				return ErrInvalidLengthFeeData
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthFeeData
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.FromAddress = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Fee", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
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
				return ErrInvalidLengthFeeData
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthFeeData
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.Fee.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 4:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field FeeType", wireType)
			}
			m.FeeType = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.FeeType |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 5:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field BlockHeight", wireType)
			}
			m.BlockHeight = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.BlockHeight |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 6:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Memo", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowFeeData
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
				return ErrInvalidLengthFeeData
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthFeeData
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Memo = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipFeeData(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthFeeData
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
func skipFeeData(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowFeeData
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
					return 0, ErrIntOverflowFeeData
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
					return 0, ErrIntOverflowFeeData
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
				return 0, ErrInvalidLengthFeeData
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupFeeData
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthFeeData
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthFeeData        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowFeeData          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupFeeData = fmt.Errorf("proto: unexpected end of group")
)
