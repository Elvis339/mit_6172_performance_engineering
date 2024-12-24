GOOS?=darwin
GOARCH?=arm64
GO=go

LDFLAGS=-w -s
GCFLAGS=-B -l=4

PROGRAM?=isort

.PHONY: all clean run run-fast build build-fast

all: build

build:
	# -gcflags="-l=4 -N" build with debug and inline max
	GOOS=${GOOS} GOARCH=${GOARCH} ${GO} build -gcflags="-l=4 -N" -o bin/${PROGRAM}

build-fast:
	GOOS=${GOOS} GOARCH=${GOARCH} ${GO} build -ldflags="${LDFLAGS}" -gcflags="${GCFLAGS}" -o bin/${PROGRAM}-fast

run: build
	./bin/${PROGRAM}

run-fast: build-fast
	./bin/${PROGRAM}-fast

clean:
	rm -rf bin/
	mkdir -p bin/

init:
	mkdir -p bin/