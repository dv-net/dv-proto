package eproxy

import (
	"context"

	"connectrpc.com/connect"
)

type ClientInterceptor struct {
	appVersion string
}

func NewClientInterceptor(appVersion string) *ClientInterceptor {
	return &ClientInterceptor{
		appVersion: appVersion,
	}
}

// WrapUnary returns a new unary server interceptor.
func (i *ClientInterceptor) WrapUnary(next connect.UnaryFunc) connect.UnaryFunc {
	return connect.UnaryFunc(func(
		ctx context.Context,
		req connect.AnyRequest,
	) (connect.AnyResponse, error) {
		req.Header().Set("version", i.appVersion)
		return next(ctx, req)
	})
}

// WrapStreamingHandler returns a new stream server interceptor.
func (i *ClientInterceptor) WrapStreamingHandler(next connect.StreamingHandlerFunc) connect.StreamingHandlerFunc {
	return connect.StreamingHandlerFunc(func(
		ctx context.Context,
		conn connect.StreamingHandlerConn,
	) error {
		conn.RequestHeader().Set("version", i.appVersion)
		return next(ctx, conn)
	})
}

func (i *ClientInterceptor) WrapStreamingClient(next connect.StreamingClientFunc) connect.StreamingClientFunc {
	return connect.StreamingClientFunc(func(
		ctx context.Context,
		spec connect.Spec,
	) connect.StreamingClientConn {
		conn := next(ctx, spec)
		conn.RequestHeader().Set("version", i.appVersion)

		return conn
	})
}
