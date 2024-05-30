package main

import (
    "log"
    "mime"
    "net/http"
    "path/filepath"
)

func main() {
    // Set MIME types for additional file types
    mime.AddExtensionType(".wasm", "application/wasm")
    mime.AddExtensionType(".mp3", "audio/mpeg")
    mime.AddExtensionType(".wav", "audio/wav")

    // Serve files with the correct MIME type and handle CORS
    fs := http.FileServer(http.Dir("./static"))
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        w.Header().Set("Access-Control-Allow-Origin", "*")
        if r.Method == http.MethodOptions {
            w.Header().Set("Access-Control-Allow-Methods", "GET, OPTIONS")
            w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
            w.WriteHeader(http.StatusOK)
            return
        }
        ext := filepath.Ext(r.URL.Path)
        if ext == ".wasm" || ext == ".mp3" || ext == ".wav" {
            w.Header().Set("Content-Type", mime.TypeByExtension(ext))
        }
        fs.ServeHTTP(w, r)
    })

    log.Println("Listening on :8080...")
    err := http.ListenAndServe(":8080", nil)
    if err != nil {
        log.Fatal(err)
    }
}
