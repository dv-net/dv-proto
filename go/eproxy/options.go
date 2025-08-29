package eproxy

import (
	"net/http"

	"connectrpc.com/connect"
)

type Option func(*Options)

type Options struct {
	httpClient     *http.Client
	connectrpcOpts []connect.ClientOption
	version        string
}

// WithVersion sets the version.
func WithVersion(version string) Option {
	return func(o *Options) {
		o.version = version
	}
}

// WithHTTPClient sets the http client.
func WithHTTPClient(client *http.Client) Option {
	return func(o *Options) {
		o.httpClient = client
	}
}

// WithConnectrpcOpts sets the connectrpc options.
func WithConnectrpcOpts(opts ...connect.ClientOption) Option {
	return func(o *Options) {
		o.connectrpcOpts = append(o.connectrpcOpts, opts...)
	}
}
