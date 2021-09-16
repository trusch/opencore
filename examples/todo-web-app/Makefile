run: build
	podman run \
		--add-host localhost:127.0.0.1 \
		--rm \
		-d \
		--name caddy \
		--replace \
		-p 8080:80 \
		-v ./build:/usr/share/caddy caddy

build: node_modules src/catalog_pb.ts src/*.tsx src/*.ts
	yarn run build

node_modules:
	yarn install

src/catalog_pb.ts: ../../proto/*.proto
	protoc \
		-I=../../proto \
		catalog.proto \
		idp.proto \
		--js_out=import_style=commonjs:./src \
		--grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:./src
		