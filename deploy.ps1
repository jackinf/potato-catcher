# gsutil mb -l europe-west4 gs://potato-catcher-wasm/
# gsutil cp static/potato-catcher_bg.wasm gs://potato-catcher-wasm/
# gsutil iam ch allUsers:objectViewer gs://potato-catcher-wasm
# gsutil cors set cors-config.json gs://potato-catcher-wasm

docker build -t potato-catcher .

# gcloud auth configure-docker

docker tag potato-catcher gcr.io/rust-gamedev/potato-catcher

docker push gcr.io/rust-gamedev/potato-catcher

gcloud run deploy potato-catcher --image gcr.io/rust-gamedev/potato-catcher --platform managed --region europe-west4 --allow-unauthenticated
