// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: epochs/query.proto

package types

import (
	context "context"
	fmt "fmt"
	_ "github.com/gogo/protobuf/gogoproto"
	grpc1 "github.com/gogo/protobuf/grpc"
	proto "github.com/gogo/protobuf/proto"
	_ "google.golang.org/genproto/googleapis/api/annotations"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
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

type QueryEpochsInfoRequest struct {
}

func (m *QueryEpochsInfoRequest) Reset()         { *m = QueryEpochsInfoRequest{} }
func (m *QueryEpochsInfoRequest) String() string { return proto.CompactTextString(m) }
func (*QueryEpochsInfoRequest) ProtoMessage()    {}
func (*QueryEpochsInfoRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e760c2f82b90e24, []int{0}
}
func (m *QueryEpochsInfoRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *QueryEpochsInfoRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_QueryEpochsInfoRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *QueryEpochsInfoRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_QueryEpochsInfoRequest.Merge(m, src)
}
func (m *QueryEpochsInfoRequest) XXX_Size() int {
	return m.Size()
}
func (m *QueryEpochsInfoRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_QueryEpochsInfoRequest.DiscardUnknown(m)
}

var xxx_messageInfo_QueryEpochsInfoRequest proto.InternalMessageInfo

type QueryEpochsInfoResponse struct {
	Epochs []EpochInfo `protobuf:"bytes,1,rep,name=epochs,proto3" json:"epochs"`
}

func (m *QueryEpochsInfoResponse) Reset()         { *m = QueryEpochsInfoResponse{} }
func (m *QueryEpochsInfoResponse) String() string { return proto.CompactTextString(m) }
func (*QueryEpochsInfoResponse) ProtoMessage()    {}
func (*QueryEpochsInfoResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e760c2f82b90e24, []int{1}
}
func (m *QueryEpochsInfoResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *QueryEpochsInfoResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_QueryEpochsInfoResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *QueryEpochsInfoResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_QueryEpochsInfoResponse.Merge(m, src)
}
func (m *QueryEpochsInfoResponse) XXX_Size() int {
	return m.Size()
}
func (m *QueryEpochsInfoResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_QueryEpochsInfoResponse.DiscardUnknown(m)
}

var xxx_messageInfo_QueryEpochsInfoResponse proto.InternalMessageInfo

func (m *QueryEpochsInfoResponse) GetEpochs() []EpochInfo {
	if m != nil {
		return m.Epochs
	}
	return nil
}

type QueryCurrentEpochRequest struct {
	Identifier string `protobuf:"bytes,1,opt,name=identifier,proto3" json:"identifier,omitempty"`
}

func (m *QueryCurrentEpochRequest) Reset()         { *m = QueryCurrentEpochRequest{} }
func (m *QueryCurrentEpochRequest) String() string { return proto.CompactTextString(m) }
func (*QueryCurrentEpochRequest) ProtoMessage()    {}
func (*QueryCurrentEpochRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e760c2f82b90e24, []int{2}
}
func (m *QueryCurrentEpochRequest) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *QueryCurrentEpochRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_QueryCurrentEpochRequest.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *QueryCurrentEpochRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_QueryCurrentEpochRequest.Merge(m, src)
}
func (m *QueryCurrentEpochRequest) XXX_Size() int {
	return m.Size()
}
func (m *QueryCurrentEpochRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_QueryCurrentEpochRequest.DiscardUnknown(m)
}

var xxx_messageInfo_QueryCurrentEpochRequest proto.InternalMessageInfo

func (m *QueryCurrentEpochRequest) GetIdentifier() string {
	if m != nil {
		return m.Identifier
	}
	return ""
}

type QueryCurrentEpochResponse struct {
	CurrentEpoch int64 `protobuf:"varint,1,opt,name=current_epoch,json=currentEpoch,proto3" json:"current_epoch,omitempty"`
}

func (m *QueryCurrentEpochResponse) Reset()         { *m = QueryCurrentEpochResponse{} }
func (m *QueryCurrentEpochResponse) String() string { return proto.CompactTextString(m) }
func (*QueryCurrentEpochResponse) ProtoMessage()    {}
func (*QueryCurrentEpochResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_2e760c2f82b90e24, []int{3}
}
func (m *QueryCurrentEpochResponse) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *QueryCurrentEpochResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_QueryCurrentEpochResponse.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *QueryCurrentEpochResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_QueryCurrentEpochResponse.Merge(m, src)
}
func (m *QueryCurrentEpochResponse) XXX_Size() int {
	return m.Size()
}
func (m *QueryCurrentEpochResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_QueryCurrentEpochResponse.DiscardUnknown(m)
}

var xxx_messageInfo_QueryCurrentEpochResponse proto.InternalMessageInfo

func (m *QueryCurrentEpochResponse) GetCurrentEpoch() int64 {
	if m != nil {
		return m.CurrentEpoch
	}
	return 0
}

func init() {
	proto.RegisterType((*QueryEpochsInfoRequest)(nil), "quasarlabs.quasarnode.epochs.QueryEpochsInfoRequest")
	proto.RegisterType((*QueryEpochsInfoResponse)(nil), "quasarlabs.quasarnode.epochs.QueryEpochsInfoResponse")
	proto.RegisterType((*QueryCurrentEpochRequest)(nil), "quasarlabs.quasarnode.epochs.QueryCurrentEpochRequest")
	proto.RegisterType((*QueryCurrentEpochResponse)(nil), "quasarlabs.quasarnode.epochs.QueryCurrentEpochResponse")
}

func init() { proto.RegisterFile("epochs/query.proto", fileDescriptor_2e760c2f82b90e24) }

var fileDescriptor_2e760c2f82b90e24 = []byte{
	// 392 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x94, 0x92, 0x4f, 0x4f, 0xe2, 0x40,
	0x18, 0xc6, 0x3b, 0xb0, 0x4b, 0xb2, 0xb3, 0xec, 0x65, 0x42, 0x76, 0xbb, 0x0d, 0xe9, 0xb2, 0xe5,
	0x20, 0x7a, 0xe8, 0x08, 0xfe, 0x4b, 0x3c, 0x19, 0x0c, 0x07, 0x8e, 0xf6, 0xe8, 0x45, 0xdb, 0x32,
	0x94, 0x26, 0x38, 0x53, 0x3a, 0x53, 0x23, 0x57, 0x3f, 0x81, 0x89, 0x5f, 0xc1, 0x2f, 0xe0, 0xb7,
	0xe0, 0x48, 0xe2, 0xc5, 0x93, 0x31, 0xc5, 0x0f, 0x62, 0x98, 0x8e, 0x52, 0x23, 0x21, 0x72, 0x9b,
	0xbe, 0xef, 0xfb, 0x3c, 0xcf, 0x6f, 0xde, 0x0e, 0x44, 0x24, 0x62, 0xfe, 0x80, 0xe3, 0x51, 0x42,
	0xe2, 0xb1, 0x1d, 0xc5, 0x4c, 0x30, 0x54, 0x1d, 0x25, 0x2e, 0x77, 0xe3, 0xa1, 0xeb, 0x71, 0x3b,
	0x3b, 0x52, 0xd6, 0x23, 0x76, 0x36, 0x69, 0x54, 0x02, 0x16, 0x30, 0x39, 0x88, 0xe7, 0xa7, 0x4c,
	0x63, 0x54, 0x03, 0xc6, 0x82, 0x21, 0xc1, 0x6e, 0x14, 0x62, 0x97, 0x52, 0x26, 0x5c, 0x11, 0x32,
	0xca, 0x55, 0xb7, 0xa2, 0x52, 0x02, 0x42, 0x09, 0x0f, 0x55, 0xd5, 0xd2, 0xe1, 0xef, 0x93, 0x79,
	0x6c, 0x47, 0x36, 0xbb, 0xb4, 0xcf, 0x1c, 0x32, 0x4a, 0x08, 0x17, 0xd6, 0x39, 0xfc, 0xf3, 0xa9,
	0xc3, 0x23, 0x46, 0x39, 0x41, 0x1d, 0x58, 0xca, 0xcc, 0x74, 0x50, 0x2b, 0x36, 0x7e, 0xb6, 0x36,
	0xec, 0x55, 0xb4, 0xb6, 0x74, 0x98, 0x1b, 0xb4, 0xbf, 0x4d, 0x9e, 0xfe, 0x69, 0x8e, 0x12, 0x5b,
	0x87, 0x50, 0x97, 0x09, 0xc7, 0x49, 0x1c, 0x13, 0x2a, 0xe4, 0x98, 0x4a, 0x47, 0x26, 0x84, 0x61,
	0x8f, 0x50, 0x11, 0xf6, 0x43, 0x12, 0xeb, 0xa0, 0x06, 0x1a, 0x3f, 0x9c, 0x5c, 0xc5, 0x3a, 0x82,
	0x7f, 0x97, 0x68, 0x15, 0x5f, 0x1d, 0xfe, 0xf2, 0xb3, 0xfa, 0x99, 0x8c, 0x92, 0xfa, 0xa2, 0x53,
	0xf6, 0x73, 0xc3, 0xad, 0xb4, 0x00, 0xbf, 0x4b, 0x0b, 0x74, 0x07, 0x20, 0x7c, 0x67, 0xe4, 0x68,
	0x77, 0xf5, 0x6d, 0x96, 0xaf, 0xcb, 0xd8, 0x5b, 0x53, 0x95, 0xa1, 0x5a, 0x9b, 0xd7, 0x0f, 0x2f,
	0xb7, 0x85, 0x3a, 0xfa, 0x8f, 0x17, 0x72, 0xac, 0xfe, 0xd4, 0x65, 0xd3, 0x23, 0xc2, 0x6d, 0xaa,
	0x4f, 0x74, 0x0f, 0x60, 0x39, 0x7f, 0x5d, 0xb4, 0xff, 0x85, 0xc8, 0x25, 0xbb, 0x35, 0x0e, 0xd6,
	0xd6, 0x29, 0xd8, 0x6d, 0x09, 0xbb, 0x85, 0x1a, 0x2b, 0x60, 0x3f, 0x2c, 0xbe, 0xdd, 0x9d, 0xa4,
	0x26, 0x98, 0xa6, 0x26, 0x78, 0x4e, 0x4d, 0x70, 0x33, 0x33, 0xb5, 0xe9, 0xcc, 0xd4, 0x1e, 0x67,
	0xa6, 0x76, 0x8a, 0x83, 0x50, 0x0c, 0x12, 0xcf, 0xf6, 0xd9, 0x45, 0xde, 0x6d, 0x81, 0x83, 0xaf,
	0xde, 0xac, 0xc5, 0x38, 0x22, 0xdc, 0x2b, 0xc9, 0x07, 0xbb, 0xf3, 0x1a, 0x00, 0x00, 0xff, 0xff,
	0x2e, 0xd2, 0x6f, 0x9f, 0x2e, 0x03, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// QueryClient is the client API for Query service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type QueryClient interface {
	// EpochInfos provide running epochInfos
	EpochInfos(ctx context.Context, in *QueryEpochsInfoRequest, opts ...grpc.CallOption) (*QueryEpochsInfoResponse, error)
	// CurrentEpoch provide current epoch of specified identifier
	CurrentEpoch(ctx context.Context, in *QueryCurrentEpochRequest, opts ...grpc.CallOption) (*QueryCurrentEpochResponse, error)
}

type queryClient struct {
	cc grpc1.ClientConn
}

func NewQueryClient(cc grpc1.ClientConn) QueryClient {
	return &queryClient{cc}
}

func (c *queryClient) EpochInfos(ctx context.Context, in *QueryEpochsInfoRequest, opts ...grpc.CallOption) (*QueryEpochsInfoResponse, error) {
	out := new(QueryEpochsInfoResponse)
	err := c.cc.Invoke(ctx, "/quasarlabs.quasarnode.epochs.Query/EpochInfos", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *queryClient) CurrentEpoch(ctx context.Context, in *QueryCurrentEpochRequest, opts ...grpc.CallOption) (*QueryCurrentEpochResponse, error) {
	out := new(QueryCurrentEpochResponse)
	err := c.cc.Invoke(ctx, "/quasarlabs.quasarnode.epochs.Query/CurrentEpoch", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// QueryServer is the server API for Query service.
type QueryServer interface {
	// EpochInfos provide running epochInfos
	EpochInfos(context.Context, *QueryEpochsInfoRequest) (*QueryEpochsInfoResponse, error)
	// CurrentEpoch provide current epoch of specified identifier
	CurrentEpoch(context.Context, *QueryCurrentEpochRequest) (*QueryCurrentEpochResponse, error)
}

// UnimplementedQueryServer can be embedded to have forward compatible implementations.
type UnimplementedQueryServer struct {
}

func (*UnimplementedQueryServer) EpochInfos(ctx context.Context, req *QueryEpochsInfoRequest) (*QueryEpochsInfoResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method EpochInfos not implemented")
}
func (*UnimplementedQueryServer) CurrentEpoch(ctx context.Context, req *QueryCurrentEpochRequest) (*QueryCurrentEpochResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CurrentEpoch not implemented")
}

func RegisterQueryServer(s grpc1.Server, srv QueryServer) {
	s.RegisterService(&_Query_serviceDesc, srv)
}

func _Query_EpochInfos_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(QueryEpochsInfoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).EpochInfos(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/quasarlabs.quasarnode.epochs.Query/EpochInfos",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).EpochInfos(ctx, req.(*QueryEpochsInfoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Query_CurrentEpoch_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(QueryCurrentEpochRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(QueryServer).CurrentEpoch(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/quasarlabs.quasarnode.epochs.Query/CurrentEpoch",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(QueryServer).CurrentEpoch(ctx, req.(*QueryCurrentEpochRequest))
	}
	return interceptor(ctx, in, info, handler)
}

var _Query_serviceDesc = grpc.ServiceDesc{
	ServiceName: "quasarlabs.quasarnode.epochs.Query",
	HandlerType: (*QueryServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "EpochInfos",
			Handler:    _Query_EpochInfos_Handler,
		},
		{
			MethodName: "CurrentEpoch",
			Handler:    _Query_CurrentEpoch_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "epochs/query.proto",
}

func (m *QueryEpochsInfoRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *QueryEpochsInfoRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *QueryEpochsInfoRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	return len(dAtA) - i, nil
}

func (m *QueryEpochsInfoResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *QueryEpochsInfoResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *QueryEpochsInfoResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Epochs) > 0 {
		for iNdEx := len(m.Epochs) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.Epochs[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintQuery(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0xa
		}
	}
	return len(dAtA) - i, nil
}

func (m *QueryCurrentEpochRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *QueryCurrentEpochRequest) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *QueryCurrentEpochRequest) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.Identifier) > 0 {
		i -= len(m.Identifier)
		copy(dAtA[i:], m.Identifier)
		i = encodeVarintQuery(dAtA, i, uint64(len(m.Identifier)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func (m *QueryCurrentEpochResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *QueryCurrentEpochResponse) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *QueryCurrentEpochResponse) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.CurrentEpoch != 0 {
		i = encodeVarintQuery(dAtA, i, uint64(m.CurrentEpoch))
		i--
		dAtA[i] = 0x8
	}
	return len(dAtA) - i, nil
}

func encodeVarintQuery(dAtA []byte, offset int, v uint64) int {
	offset -= sovQuery(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *QueryEpochsInfoRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	return n
}

func (m *QueryEpochsInfoResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if len(m.Epochs) > 0 {
		for _, e := range m.Epochs {
			l = e.Size()
			n += 1 + l + sovQuery(uint64(l))
		}
	}
	return n
}

func (m *QueryCurrentEpochRequest) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Identifier)
	if l > 0 {
		n += 1 + l + sovQuery(uint64(l))
	}
	return n
}

func (m *QueryCurrentEpochResponse) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if m.CurrentEpoch != 0 {
		n += 1 + sovQuery(uint64(m.CurrentEpoch))
	}
	return n
}

func sovQuery(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozQuery(x uint64) (n int) {
	return sovQuery(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *QueryEpochsInfoRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowQuery
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
			return fmt.Errorf("proto: QueryEpochsInfoRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: QueryEpochsInfoRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipQuery(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthQuery
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
func (m *QueryEpochsInfoResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowQuery
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
			return fmt.Errorf("proto: QueryEpochsInfoResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: QueryEpochsInfoResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Epochs", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowQuery
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
				return ErrInvalidLengthQuery
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthQuery
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Epochs = append(m.Epochs, EpochInfo{})
			if err := m.Epochs[len(m.Epochs)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipQuery(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthQuery
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
func (m *QueryCurrentEpochRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowQuery
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
			return fmt.Errorf("proto: QueryCurrentEpochRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: QueryCurrentEpochRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Identifier", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowQuery
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
				return ErrInvalidLengthQuery
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthQuery
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Identifier = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipQuery(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthQuery
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
func (m *QueryCurrentEpochResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowQuery
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
			return fmt.Errorf("proto: QueryCurrentEpochResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: QueryCurrentEpochResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field CurrentEpoch", wireType)
			}
			m.CurrentEpoch = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowQuery
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.CurrentEpoch |= int64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipQuery(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthQuery
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
func skipQuery(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowQuery
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
					return 0, ErrIntOverflowQuery
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
					return 0, ErrIntOverflowQuery
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
				return 0, ErrInvalidLengthQuery
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupQuery
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthQuery
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthQuery        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowQuery          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupQuery = fmt.Errorf("proto: unexpected end of group")
)
