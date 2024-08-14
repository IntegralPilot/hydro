CC = clang
CFLAGS = -nostdlib --target=wasm32 -Ofast
LDFLAGS = -Wl,--allow-undefined

all: main.wasm

main.wasm:
	$(CC) $(CFLAGS) $(LDFLAGS) src/hello.c -o ./main.wasm

clean:
	del main.wasm
