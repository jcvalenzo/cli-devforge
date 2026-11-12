.PHONY: build run test clean docker

build:
	go build -o bin/sample ./cmd/server

run:
	go run ./cmd/server

test:
	go test ./...

clean:
	rm -rf bin/

docker:
	docker build -t ms-sample:latest .
