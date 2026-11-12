package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/acme/catalog/internal/handler"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /", handler.Health)
	mux.HandleFunc("GET /api/v1/catalog", handler.List)
	mux.HandleFunc("POST /api/v1/catalog", handler.Create)

	addr := ":8080"
	fmt.Printf("API %s listening on %s\n", "api-catalog", addr)
	log.Fatal(http.ListenAndServe(addr, mux))
}
