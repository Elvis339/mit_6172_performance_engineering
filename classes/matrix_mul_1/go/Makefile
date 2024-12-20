BINARY_NAME=matmul
GO=go

# Default target OS and architecture (M1)
GOOS?=darwin
GOARCH?=arm64

#-w              # Disables DWARF debugging information
#-s              # Disables symbol table
## Together, -w -s reduce binary size and strip debug info
LDFLAGS=-w -s

#-B              # Disables bounds checking
#                # Very unsafe but faster - use with caution!

#-l=4            # Sets aggressive function inlining level
#                # Default is -l=0, higher numbers = more aggressive inlining
#                # 4 is currently the maximum practical level
GCFLAGS=-B -l=4

# Safe run
run-safe:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run main.go \
		-trimpath \
		-ldflags="$(LDFLAGS)" \
		-o $(BINARY_NAME)

# Performance (unsafe)
run-fast:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run main.go \
		-trimpath \
		-ldflags="$(LDFLAGS)" \
		-gcflags="$(GCFLAGS)"


.PHONY: build build-safe build-fast clean test bench fmt run run-fast