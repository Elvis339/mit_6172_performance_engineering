.PHONY: run-release run-debug llvm asm

RUST_FLAGS :=
OPT_LEVEL := 3

run-release:
	RUSTFLAGS="$(RUST_FLAGS) -C opt-level=$(OPT_LEVEL)" cargo run --release

run-debug:
	RUSTFLAGS="$(RUST_FLAGS)" cargo run --debug

llvm:
	RUSTFLAGS="--emit=llvm-ir -C opt-level=$(OPT_LEVEL)" cargo build
	@echo "LLVM IR files in target/debug/deps/*.ll"

asm:
	RUSTFLAGS="--emit=asm -C opt-level=$(OPT_LEVEL)" cargo build
	@echo "Assembly files in target/debug/deps/*.s"

clean:
	cargo clean