CC = /usr/bin/clang
CFLAGS = -O2
TARGET = $(PROGRAM).build
SOURCE = $(PROGRAM).c

.PHONY: clean run

PROGRAM ?= main

run: $(TARGET)
	./$(TARGET)

$(TARGET): $(SOURCE)
	$(CC) $(SOURCE) -o $(TARGET) $(CFLAGS)
	@echo "$(PROGRAM) compiled successfully!"

clean:
	rm -rf *.build