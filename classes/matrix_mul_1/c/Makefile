CC = /opt/opencilk/bin/clang
CFLAGS = -O3 -fopencilk
SYSROOT = $(shell xcrun --show-sdk-path)
TARGET = main.build
SOURCE = main.c

.PHONY: clean run

run: $(TARGET)
	./$(TARGET)

$(TARGET): $(SOURCE)
	$(CC) $(SOURCE) -o $(TARGET) $(CFLAGS) -isysroot $(SYSROOT)
	@echo "Matrix mul compiled successfully!"

clean:
	rm -rf $(TARGET)