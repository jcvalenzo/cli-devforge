.PHONY: build test install uninstall clean all

build:
	go build -o bin/devforge .

test:
	go test ./...

install:
	go install .

uninstall:
	rm -f $(shell go env GOPATH)/bin/cli-devforge

clean:
	rm -rf bin/

all: clean build test
