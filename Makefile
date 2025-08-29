lint:
	golangci-lint run --show-stats

genproto:
	rm -rf ./gen ./api
	buf lint
	# eproxy
	buf generate --path schema/eproxy --template buf.gen.yaml
	mv api/api.swagger.json api/eproxy.swagger.json
	# manager
	buf generate --path schema/manager --template buf.gen.yaml
	mv api/api.swagger.json api/manager.swagger.json
	# watcher
	buf generate --path schema/watcher --template buf.gen.yaml
	mv api/api.swagger.json api/watcher.swagger.json



install-dev-tools:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest && protoc-gen-go --version
	go install github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2@latest && protoc-gen-openapiv2 --version
	go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@latest && protoc-gen-doc --version
	go install connectrpc.com/connect/cmd/protoc-gen-connect-go@latest && protoc-gen-connect-go --version
