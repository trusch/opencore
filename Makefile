IMAGE=containers.trusch.io/opencore/core:latest
BASE_IMAGE=gcr.io/distroless/cc-debian11:latest
BUILD_IMAGE=containers.trusch.io/opencore/builder
BUILD_BASE_IMAGE=docker.io/library/rust:1.57-bullseye
DB_VOLUME=--tmpfs=/var/lib/postgresql/data

# use mold for faster linking
# disable by setting MOLD=
MOLD=/mold/mold -run

image: .image

bin/opencore: $(shell find ./ -name "*.rs") $(shell find ./ -name "*.proto") .build-image
	podman run -it --rm -v .:/app -w /app -v cargo-cache:/usr/local/cargo $(BUILD_IMAGE) $(MOLD) cargo build --release
	mkdir -p bin
	cp target/release/opencore bin/

.image: bin/opencore Makefile
	$(eval ID=$(shell buildah from $(BASE_IMAGE)))
	buildah copy $(ID) bin/opencore /bin/opencore
	buildah config --user 3001:3001 $(ID) 
	buildah config --cmd "/bin/opencore" $(ID) 
	buildah commit $(ID) $(IMAGE)
	buildah rm $(ID)
	touch .image

.build-image:
	$(eval ID=$(shell buildah from $(BUILD_BASE_IMAGE)))
	buildah run $(ID) -- rustup component add rustfmt
	buildah run $(ID) -- git clone https://github.com/rui314/mold.git /mold
	buildah run $(ID) -- apt update
	buildah run $(ID) -- apt install -y build-essential clang lld cmake libxxhash-dev libssl-dev
	buildah run $(ID) -- make -C /mold -j8
	buildah commit $(ID) $(BUILD_IMAGE)
	buildah rm $(ID)
	touch .build-image

DOCKER?=docker
docker-build: $(shell find ./ -name "*.rs") $(shell find ./ -name "*.proto")
	$(DOCKER) build -t $(IMAGE) .
	touch .image

run: .image
	-podman pod kill opencore
	podman pod create --replace --name opencore \
		-p 0.0.0.0:3001:3001 \
		-p 0.0.0.0:3002:3002 \
		-p 0.0.0.0:5432:5432 \
		-p 6831:6831/udp \
		-p 6832:6832/udp \
		-p 16686:16686
	podman run -d --pod opencore --name postgres \
		-e POSTGRES_PASSWORD=password \
		--add-host localhost:127.0.0.1 \
		$(DB_VOLUME) \
		postgres:latest -c log_statement=all
	podman run -d --pod opencore --name jaeger \
		jaegertracing/all-in-one:latest
	podman run -d --pod opencore --name core \
		-e RUST_LOG=info \
		-e RUST_BACKTRACE=1 \
		-v ./examples/todo-web-app/build:/static \
		${IMAGE} /bin/opencore \
			--listen 0.0.0.0:3001 \
			--database "postgres://postgres:password@127.0.0.1" \
			--secret secret \
			--static-dir /static

DB_VOLUME_NAME="opencore-data"
run-persistent:
	$(MAKE) run DB_VOLUME="-v $(DB_VOLUME_NAME):/var/lib/postgresql/data"

run-with-list-test-volume: .image
	$(MAKE) run-persistent DB_VOLUME_NAME="opencore-list-test"

stop:
	podman pod rm -f opencore
