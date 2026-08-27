VERSION=$(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
LDFLAGS=-s -w -X github.com/jcvalenzo/cli-devforge/cmd.version=$(VERSION)

.PHONY: build build-all test install uninstall clean all lint fmt vet coverage

build:
	go build -ldflags "$(LDFLAGS)" -o bin/devforge .

build-all:
	mkdir -p dist
	GOOS=linux GOARCH=amd64 go build -ldflags "$(LDFLAGS)" -o dist/devforge-linux-amd64 .
	GOOS=linux GOARCH=arm64 go build -ldflags "$(LDFLAGS)" -o dist/devforge-linux-arm64 .
	GOOS=darwin GOARCH=amd64 go build -ldflags "$(LDFLAGS)" -o dist/devforge-darwin-amd64 .
	GOOS=darwin GOARCH=arm64 go build -ldflags "$(LDFLAGS)" -o dist/devforge-darwin-arm64 .
	GOOS=windows GOARCH=amd64 go build -ldflags "$(LDFLAGS)" -o dist/devforge-windows-amd64.exe .

test:
	go test -v -coverprofile=coverage.out ./...

vet:
	go vet ./...

fmt:
	gofmt -s -w .
	@test -z "$$(gofmt -l .)" || (echo "Code not formatted. Run 'make fmt'" && exit 1)

lint:
	golangci-lint run

coverage:
	go test -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html

install:
	go install -ldflags "$(LDFLAGS)" .

uninstall:
	rm -f $(shell go env GOPATH)/bin/cli-devforge

clean:
	rm -rf bin/ dist/ coverage.out coverage.html

all: clean fmt vet lint build test
