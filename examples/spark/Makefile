IMAGE_BASE_NAME=containers.trusch.io/opencore/spark

run: .image
	podman run -d --pod opencore -it --name spark-worker $(IMAGE_BASE_NAME)/worker:latest \
		spark-submit --jars "/app/jars/*.jar" /app/main.py \
			--opencore_addr=localhost:3001 \
			--service_account_id=data \
			--service_account_token='$(DATA_SA_KEY)'

image: .image
.image: main.py .base-image Makefile grpc
	$(eval ID=$(shell buildah from $(IMAGE_BASE_NAME)/base:latest))
	buildah copy $(ID) *.py /app/
	buildah copy $(ID) *.jar /app/jars/
	buildah commit $(ID) $(IMAGE_BASE_NAME)/worker:latest
	buildah rm $(ID)
	touch .image

.base-image:
	$(eval ID=$(shell buildah from docker.io/bitnami/spark:3-debian-10))
	buildah run --user root:root $(ID) python -m pip install --upgrade pip
	buildah run --user root:root $(ID) python -m pip install numpy grpcio grpcio-tools
	buildah config --cmd 'spark-submit --jars "/app/jars/*.jar" /app/main.py' $(ID)
	buildah commit $(ID) $(IMAGE_BASE_NAME)/base:latest
	buildah rm $(ID)
	touch .base-image

grpc: catalog_pb2.py
catalog_pb2.py:
	podman run --user root:root -v ../../:/opencore -v .:/app -w /app $(IMAGE_BASE_NAME)/base:latest \
		python -m grpc.tools.protoc -I/opencore/proto --python_out=. --grpc_python_out=. /opencore/proto/idp.proto /opencore/proto/catalog.proto
