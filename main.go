package main

import (
    "mime"
    "net/http"
    "path/filepath"
    "log"
)

func main() {
    // Set the MIME type for .wasm files
    mime.AddExtensionType(".wasm", "application/wasm")

    // Set the directory where static files are located
    fs := http.FileServer(http.Dir("./static"))

    // Serve files with the correct MIME type
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        if filepath.Ext(r.URL.Path) == ".wasm" {
            w.Header().Set("Content-Type", "application/wasm")
        }
        fs.ServeHTTP(w, r)
    })

    log.Println("Listening on :8080...")
    err := http.ListenAndServe(":8080", nil)
    if err != nil {
        log.Fatal(err)
    }
}
