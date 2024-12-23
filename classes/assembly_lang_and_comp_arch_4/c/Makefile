CC = /usr/bin/clang
CFLAGS = -O3
TARGET = $(PROGRAM).build
SOURCE = $(PROGRAM).c

.PHONY: clean run asm disasm debug

PROGRAM ?= main

run: $(TARGET)
	./$(TARGET)

asm:
	$(CC) $(SOURCE) $(CFLAGS) -S
	@echo "$(PROGRAM) produced assembly successfully!"

disasm: $(TARGET)
	objdump -S $(TARGET)

debug:
	$(CC) $(SOURCE) -o $(TARGET) $(CFLAGS) -g
	@echo "$(PROGRAM) compiled successfully with debug flag"

$(TARGET): $(SOURCE)
	$(CC) $(SOURCE) -o $(TARGET) $(CFLAGS)
	@echo "$(PROGRAM) compiled successfully!"

clean:
	rm -rf *.build *.s