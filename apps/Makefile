CC = clang
CFLAGS = -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	$(CC) $(CFLAGS) $(LDFLAGS) hello.c -o ./hello.wasm

clean:
	del main.wasm
