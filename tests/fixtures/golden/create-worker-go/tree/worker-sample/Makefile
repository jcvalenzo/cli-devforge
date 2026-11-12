.PHONY: build run test clean

build:
	go build -o bin/sample ./cmd/worker

run:
	go run ./cmd/worker

test:
	go test ./...

clean:
	rm -rf bin/
