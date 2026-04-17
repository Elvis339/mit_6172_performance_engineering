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

NAIVE=./cmd/naive
OPT=./cmd/opt

# Safe run
run-naive-safe:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run $(NAIVE) \
	   -trimpath \
	   -ldflags="$(LDFLAGS)"

run-opt-safe:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run $(OPT) \
	   -trimpath \
	   -ldflags="$(LDFLAGS)"

# Performance (unsafe)
run-naive-fast:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run $(NAIVE) \
	   -trimpath \
	   -ldflags="$(LDFLAGS)" \
	   -gcflags="$(GCFLAGS)"

run-opt-fast:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) run $(OPT) \
	   -trimpath \
	   -ldflags="$(LDFLAGS)" \
	   -gcflags="$(GCFLAGS)"


.PHONY: run-naive-safe run-opt-safe run-naive-fast run-opt-fast