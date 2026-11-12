package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/example/sample/internal/handler"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /", handler.Health)
	mux.HandleFunc("GET /api/v1/sample", handler.List)
	mux.HandleFunc("POST /api/v1/sample", handler.Create)

	addr := ":8080"
	fmt.Printf("API %s listening on %s\n", "api-sample", addr)
	log.Fatal(http.ListenAndServe(addr, mux))
}
