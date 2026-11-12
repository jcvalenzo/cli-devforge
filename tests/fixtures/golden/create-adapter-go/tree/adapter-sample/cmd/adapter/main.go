package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	fmt.Println("Adapter adapter-sample started")

	from := "sample-source"
	to := "sample-target"
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		for {
			// TODO: Implement adaptation logic for external service
			fmt.Printf("Adapting %s to %s\n", from, to)
		}
	}()

	sig := <-sigCh
	fmt.Printf("Received %s, shutting down...\n", sig)
}
