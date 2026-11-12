package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/example/sample/internal/handler"
)

func main() {
	http.HandleFunc("/", handler.Home)

	addr := ":8080"
	fmt.Printf("Starting %s on %s\n", "app-sample", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
