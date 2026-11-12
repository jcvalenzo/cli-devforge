.PHONY: build run test clean

build:
	go build -o bin/catalog ./cmd/server

run:
	go run ./cmd/server

test:
	go test ./...

clean:
	rm -rf bin/
