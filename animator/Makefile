CC=gcc
# CFLAGS=-O0 -ggdb -DDEBUG -c -std=gnu11
CFLAGS=-O2 -c -std=gnu11
CFLAGS += `sdl2-config --cflags` `pkg-config --cflags librsvg-2.0`
LDFLAGS += `sdl2-config --libs` `pkg-config --libs SDL2_ttf` `pkg-config --libs librsvg-2.0`
CPP_SOURCES=$(wildcard *.c sdl/*.c)
OBJECTS=$(CPP_SOURCES:.c=.o)
EXECUTABLE=animator

all: $(CPP_SOURCES) $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(OBJECTS) -o $@ $(LDFLAGS)

$(OBJECTS):
	$(CC) $(CFLAGS) $(@:.o=.c) -o $@

clean:
	rm -f $(OBJECTS) $(EXECUTABLE)
