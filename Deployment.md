# Deployment

## Prerequisites

You need to have these installed on your machine:
- GCP account & `glcoud`
- Rust & Cargo

## Via Makefile

On Windows, prepare `make`:
```bash
choco install make
```

Update global variables in the `Makefile`:
```makefile
PROJECT_ID=rust-gamedev
REGION=europe-west4
BUCKET_NAME=gs://potato-catcher-wasm/
IMAGE_NAME=potato-catcher
IMAGE_URI=gcr.io/$(PROJECT_ID)/$(IMAGE_NAME)
WASM_TARGET=wasm32-unknown-unknown
WASM_DIR=out
```

Run setup
```bash
make setup
```

Deploy the web server with:
```bash
make full  # or just `make`
````

## Manually

### GCloud preparation

Log into GCP with `gcloud auth login`. Set the according project
```bash
gcloud auth login
gcloud projects list
gcloud config set project rust-gamedev
```

Generate WASM & frontend files for it

### GCP Bucket preparation

We cannot have WASM file served directly from GCP Cloud Run because of the size limit. We need to serve it from GCP Cloud Storage.
Prepare the bucket with the following commands:
```bash
gsutil mb -l europe-west4 gs://potato-catcher-wasm/
gsutil iam ch allUsers:objectViewer gs://potato-catcher-wasm
gsutil cors set cors-config.json gs://potato-catcher-wasm
```

### Updating WASM file

The game needs to be compiled into WASM format to be able to run in the frontend.
To do that, we need to compile the game with `wasm-bindgen` and upload the resulting file to GCP Cloud Storage.

Install the `wasm-bindgen` tool:
```bash
cargo install wasm-bindgen-cli
```

Run the upload script:
```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/potato-catcher.wasm --out-dir out --web
gsutil cp out/potato-catcher_bg.wasm gs://potato-catcher-wasm/
```

### Deploy web server

Prepare the Docker image and deploy it to GCP Cloud Run (run only once)
```bash
gcloud auth configure-docker
```

Build the Docker image and deploy it to GCP Cloud Run
```bash
docker build -t potato-catcher .
docker tag potato-catcher gcr.io/rust-gamedev/potato-catcher
docker push gcr.io/rust-gamedev/potato-catcher
gcloud run deploy potato-catcher --image gcr.io/rust-gamedev/potato-catcher --platform managed --region europe-west4 --allow-unauthenticated
```