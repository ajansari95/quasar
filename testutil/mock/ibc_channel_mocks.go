// Code generated by MockGen. DO NOT EDIT.
// Source: github.com/abag/quasarnode/x/qoracle/types (interfaces: ChannelKeeper)

// Package mock is a generated GoMock package.
package mock

import (
	types "github.com/cosmos/cosmos-sdk/types"
	types0 "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	exported "github.com/cosmos/ibc-go/v3/modules/core/exported"
	gomock "github.com/golang/mock/gomock"
	reflect "reflect"
)

// MockChannelKeeper is a mock of ChannelKeeper interface
type MockChannelKeeper struct {
	ctrl     *gomock.Controller
	recorder *MockChannelKeeperMockRecorder
}

// MockChannelKeeperMockRecorder is the mock recorder for MockChannelKeeper
type MockChannelKeeperMockRecorder struct {
	mock *MockChannelKeeper
}

// NewMockChannelKeeper creates a new mock instance
func NewMockChannelKeeper(ctrl *gomock.Controller) *MockChannelKeeper {
	mock := &MockChannelKeeper{ctrl: ctrl}
	mock.recorder = &MockChannelKeeperMockRecorder{mock}
	return mock
}

// EXPECT returns an object that allows the caller to indicate expected use
func (m *MockChannelKeeper) EXPECT() *MockChannelKeeperMockRecorder {
	return m.recorder
}

// GetChannel mocks base method
func (m *MockChannelKeeper) GetChannel(arg0 types.Context, arg1, arg2 string) (types0.Channel, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetChannel", arg0, arg1, arg2)
	ret0, _ := ret[0].(types0.Channel)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetChannel indicates an expected call of GetChannel
func (mr *MockChannelKeeperMockRecorder) GetChannel(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetChannel", reflect.TypeOf((*MockChannelKeeper)(nil).GetChannel), arg0, arg1, arg2)
}

// GetChannelClientState mocks base method
func (m *MockChannelKeeper) GetChannelClientState(arg0 types.Context, arg1, arg2 string) (string, exported.ClientState, error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetChannelClientState", arg0, arg1, arg2)
	ret0, _ := ret[0].(string)
	ret1, _ := ret[1].(exported.ClientState)
	ret2, _ := ret[2].(error)
	return ret0, ret1, ret2
}

// GetChannelClientState indicates an expected call of GetChannelClientState
func (mr *MockChannelKeeperMockRecorder) GetChannelClientState(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetChannelClientState", reflect.TypeOf((*MockChannelKeeper)(nil).GetChannelClientState), arg0, arg1, arg2)
}

// GetChannelConnection mocks base method
func (m *MockChannelKeeper) GetChannelConnection(arg0 types.Context, arg1, arg2 string) (string, exported.ConnectionI, error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetChannelConnection", arg0, arg1, arg2)
	ret0, _ := ret[0].(string)
	ret1, _ := ret[1].(exported.ConnectionI)
	ret2, _ := ret[2].(error)
	return ret0, ret1, ret2
}

// GetChannelConnection indicates an expected call of GetChannelConnection
func (mr *MockChannelKeeperMockRecorder) GetChannelConnection(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetChannelConnection", reflect.TypeOf((*MockChannelKeeper)(nil).GetChannelConnection), arg0, arg1, arg2)
}

// GetConnection mocks base method
func (m *MockChannelKeeper) GetConnection(arg0 types.Context, arg1 string) (exported.ConnectionI, error) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetConnection", arg0, arg1)
	ret0, _ := ret[0].(exported.ConnectionI)
	ret1, _ := ret[1].(error)
	return ret0, ret1
}

// GetConnection indicates an expected call of GetConnection
func (mr *MockChannelKeeperMockRecorder) GetConnection(arg0, arg1 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetConnection", reflect.TypeOf((*MockChannelKeeper)(nil).GetConnection), arg0, arg1)
}

// GetNextSequenceSend mocks base method
func (m *MockChannelKeeper) GetNextSequenceSend(arg0 types.Context, arg1, arg2 string) (uint64, bool) {
	m.ctrl.T.Helper()
	ret := m.ctrl.Call(m, "GetNextSequenceSend", arg0, arg1, arg2)
	ret0, _ := ret[0].(uint64)
	ret1, _ := ret[1].(bool)
	return ret0, ret1
}

// GetNextSequenceSend indicates an expected call of GetNextSequenceSend
func (mr *MockChannelKeeperMockRecorder) GetNextSequenceSend(arg0, arg1, arg2 interface{}) *gomock.Call {
	mr.mock.ctrl.T.Helper()
	return mr.mock.ctrl.RecordCallWithMethodType(mr.mock, "GetNextSequenceSend", reflect.TypeOf((*MockChannelKeeper)(nil).GetNextSequenceSend), arg0, arg1, arg2)
}
