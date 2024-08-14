#ifndef _HYDRO_STREAM_H
#define _HYDRO_STREAM_H

#include <hydro/stream.c>

// These functions are implemented in the kernel
// We don't define it so that the clang makes the code dynamically link to it at runtime
int write(char* stream_name, int* buf, int count);
int *read(char* stream_name);

int autowrite(char* stream_name, int* buf);

#endif // HYDRO_STREAM_H