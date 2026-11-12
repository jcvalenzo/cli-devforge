package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/example/sample/internal/handler"
)

func main() {
	http.HandleFunc("/health", handler.Health)
	http.HandleFunc("/ready", handler.Ready)

	addr := ":8080"
	fmt.Printf("Microservice %s listening on %s\n", "ms-sample", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
