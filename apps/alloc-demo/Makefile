CC = clang
CFLAGS = -I ../../stdlib/c -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	$(CC) $(CFLAGS) $(LDFLAGS) src/alloc-demo.c -o ./main.wasm

clean:
	del main.wasm
