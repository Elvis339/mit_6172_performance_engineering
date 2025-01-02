CC = /usr/bin/clang
SRC := nbody.c
TARGET := nbody.build
OPT_LEVEL ?= 0

CFLAGS := -Wall -Wextra
LDFLAGS :=

ifdef DEBUG
	CFLAGS += -g -DDEBUG
else
	CFLAGS += -DNDEBUG
endif

.PHONY: all clean run ir asm

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -O$(OPT_LEVEL) $< -o $@ $(LDFLAGS)

ir: $(SRC)
	$(CC) $(CFLAGS) -O$(OPT_LEVEL) -S -emit-llvm $< -o $(TARGET)_O$(OPT_LEVEL).ll

asm: $(SRC)
	$(CC) $(CFLAGS) -O$(OPT_LEVEL) -S $< -o $(TARGET)_O$(OPT_LEVEL).s

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(TARGET) *.ll *.s


#make OPT_LEVEL=2           # Compile with -O2
#make DEBUG=1 OPT_LEVEL=1   # Debug build with -O1
#make ir OPT_LEVEL=3        # Generate LLVM IR with -O3
#make asm OPT_LEVEL=0       # Generate assembly with -O0