#ifndef _HYDRO_STREAM_C
#define _HYDRO_STREAM_C

#include <string.h>

// These functions are implemented in the kernel
// We don't define it so that the clang makes the code dynamically link to it at runtime
int write(char* stream_name, int* buf, int count);
int *read(char* stream_name);

int autowrite(char* stream_name, int* buf) {
    return write(stream_name, buf, strlen((char*)buf));
}

#endif // HYDRO_STREAM_C