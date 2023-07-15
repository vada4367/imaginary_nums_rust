# C common
CC = gcc
CFLAGS = -m64 -Os -g0 -s -std=c11 -Wall -Wpedantic -pedantic -Werror
CFLAGS += -Wno-strict-aliasing -Wno-missing-braces -Wno-format
CFLAGS += -I ./

# Objects
OBJS = $(addsuffix .o, $(basename $(wildcard *.c) $(wildcard *.asm)))

# Target
EXEC = complex_numbers

default : $(EXEC)

%.o : %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(EXEC) : $(OBJS)
	$(CC) $(CFLAGS) $^ -o $@