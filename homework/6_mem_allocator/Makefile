CC = clang
CFLAGS = -shared -fPIC

TARGET = libmalloc.so

SRCS = malloc.c
OBJS = $(SRCS:.c=.o)

HEADERS = malloc.h

all: $(TARGET)

# Link object files into binary
$(TARGET): $(OBJS)
	$(CC) $(OBJS) -o $(TARGET)

# Compile source files into object files
%.o: %.c $(HEADERS)
	$(CC) $(CFLAGS) -c $< -o $@

# Clean build files
clean:
	rm -f $(OBJS) $(TARGET)

# Clean and rebuild
rebuild: clean all

# Add .PHONY to declare which targets aren't real files
.PHONY: all clean rebuild