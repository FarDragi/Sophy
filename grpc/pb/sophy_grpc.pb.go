// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package pb

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// SophyClient is the client API for Sophy service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type SophyClient interface {
	AddUserGlobalXp(ctx context.Context, in *GlobalXpRequest, opts ...grpc.CallOption) (*GlobalXpResponse, error)
}

type sophyClient struct {
	cc grpc.ClientConnInterface
}

func NewSophyClient(cc grpc.ClientConnInterface) SophyClient {
	return &sophyClient{cc}
}

func (c *sophyClient) AddUserGlobalXp(ctx context.Context, in *GlobalXpRequest, opts ...grpc.CallOption) (*GlobalXpResponse, error) {
	out := new(GlobalXpResponse)
	err := c.cc.Invoke(ctx, "/sophy.Sophy/AddUserGlobalXp", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// SophyServer is the server API for Sophy service.
// All implementations must embed UnimplementedSophyServer
// for forward compatibility
type SophyServer interface {
	AddUserGlobalXp(context.Context, *GlobalXpRequest) (*GlobalXpResponse, error)
	mustEmbedUnimplementedSophyServer()
}

// UnimplementedSophyServer must be embedded to have forward compatible implementations.
type UnimplementedSophyServer struct {
}

func (UnimplementedSophyServer) AddUserGlobalXp(context.Context, *GlobalXpRequest) (*GlobalXpResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddUserGlobalXp not implemented")
}
func (UnimplementedSophyServer) mustEmbedUnimplementedSophyServer() {}

// UnsafeSophyServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to SophyServer will
// result in compilation errors.
type UnsafeSophyServer interface {
	mustEmbedUnimplementedSophyServer()
}

func RegisterSophyServer(s grpc.ServiceRegistrar, srv SophyServer) {
	s.RegisterService(&Sophy_ServiceDesc, srv)
}

func _Sophy_AddUserGlobalXp_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GlobalXpRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SophyServer).AddUserGlobalXp(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/sophy.Sophy/AddUserGlobalXp",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(SophyServer).AddUserGlobalXp(ctx, req.(*GlobalXpRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Sophy_ServiceDesc is the grpc.ServiceDesc for Sophy service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Sophy_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "sophy.Sophy",
	HandlerType: (*SophyServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "AddUserGlobalXp",
			Handler:    _Sophy_AddUserGlobalXp_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "sophy.proto",
}
