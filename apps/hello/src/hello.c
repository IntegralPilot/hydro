// declare the write function
int write(char* stream_name, int* buf, int count);

void _start() {
    // write "Hello, world!" to /dev/stdout
    char* message = "Hello, world from WebAssembly!\n";
    write("/dev/stdout", message, 30);
}