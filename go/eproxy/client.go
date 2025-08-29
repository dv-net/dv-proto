package eproxy

import (
	"errors"
	"net/http"

	"github.com/dv-net/dv-proto/gen/go/eproxy/addresses/v2/addressesv2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/assets/v2/assetsv2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/blocks/v2/blocksv2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/btclike/v2/btclikev2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/evm/v2/evmv2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/transactions/v2/transactionsv2connect"
	"github.com/dv-net/dv-proto/gen/go/eproxy/tron/v1/tronv1connect"
)

type Client struct {
	AddressesClient    addressesv2connect.AddressesServiceClient
	AssetsClient       assetsv2connect.AssetsServiceClient
	TransactionsClient transactionsv2connect.TransactionsServiceClient
	BlocksClient       blocksv2connect.BlocksServiceClient
	BTCLikeClient      btclikev2connect.BtcLikeServiceClient
	TronClient         tronv1connect.TronServiceClient
	EVMClient          evmv2connect.EVMServiceClient
}

func NewClient(baseURL string, opts ...Option) (*Client, error) {
	if baseURL == "" {
		return nil, errors.New("baseURL is required")
	}

	o := &Options{
		httpClient: http.DefaultClient,
	}

	for _, opt := range opts {
		opt(o)
	}

	// if len(o.Interceptors) > 0 {
	// 	clientIcptr := NewClientInterceptor(o.version)
	// 	icptrs := append(o.Interceptors, clientIcptr)
	// 	o.connectrpcOpts = append(o.connectrpcOpts, connect.WithInterceptors(icptrs...))
	// }

	c := &Client{
		AddressesClient:    addressesv2connect.NewAddressesServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		AssetsClient:       assetsv2connect.NewAssetsServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		TransactionsClient: transactionsv2connect.NewTransactionsServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		BlocksClient:       blocksv2connect.NewBlocksServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		BTCLikeClient:      btclikev2connect.NewBtcLikeServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		TronClient:         tronv1connect.NewTronServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
		EVMClient:          evmv2connect.NewEVMServiceClient(o.httpClient, baseURL, o.connectrpcOpts...),
	}

	return c, nil
}
