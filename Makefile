# Makefile for deploying Potato Catcher game

# Variables
PROJECT_ID=rust-gamedev
REGION=europe-west4
BUCKET_NAME=gs://potato-catcher-wasm/
IMAGE_NAME=potato-catcher
IMAGE_URI=gcr.io/$(PROJECT_ID)/$(IMAGE_NAME)
WASM_TARGET=wasm32-unknown-unknown
WASM_DIR=out

# Phony targets
.PHONY: all gcloud-auth gcloud-config bucket-setup wasm-build wasm-upload docker-auth docker-build docker-push deploy

all: wasm deploy

gcloud-auth:
	gcloud auth login

gcloud-config:
	gcloud config set project $(PROJECT_ID)

bucket-setup:
	gsutil mb -l $(REGION) $(BUCKET_NAME)
	gsutil iam ch allUsers:objectViewer $(BUCKET_NAME)
	gsutil cors set cors-config.json $(BUCKET_NAME)

wasm-build:
	cargo install wasm-bindgen-cli
	cargo build --target $(WASM_TARGET) --release
	wasm-bindgen target/$(WASM_TARGET)/release/$(IMAGE_NAME).wasm --out-dir $(WASM_DIR) --web

wasm: wasm-build
	gsutil cp $(WASM_DIR)/$(IMAGE_NAME)_bg.wasm $(BUCKET_NAME)

docker-auth:
	gcloud auth configure-docker

docker-build:
	docker build -t $(IMAGE_NAME) .
	docker tag $(IMAGE_NAME) $(IMAGE_URI)

docker-push: docker-build
	docker push $(IMAGE_URI)

deploy: docker-push
	gcloud run deploy $(IMAGE_NAME) --image $(IMAGE_URI) --platform managed --region $(REGION) --allow-unauthenticated

setup: gcloud-auth gcloud-config bucket-setup docker-auth

full: wasm deploy