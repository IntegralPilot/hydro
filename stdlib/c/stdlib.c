#ifndef _STDLIB_C
#define _STDLIB_C

#include <hydro/stream.h>
#include <stddef.h>

// malloc
// doesn't actually work for now, the used streams don't exist yet
void *malloc(size_t size) {
    write("/dev/allocator/alloc-size", (int *)&size, sizeof(size_t));
    return (void *)read("/dev/malloc/alloc-addr");
}

// free
// doesn't actually work for now, the used streams don't exist yet
void free(void *ptr) {
    write("/dev/allocator/free-addr", (int *)&ptr, sizeof(void *));
}

// realloc
// doesn't actually work for now, the used streams don't exist yet
void *realloc(void *ptr, size_t size) {
    write("/dev/allocator/realloc-addr", (int *)&ptr, sizeof(void *));
    write("/dev/allocator/alloc-size", (int *)&size, sizeof(size_t));
    return (void *)read("/dev/realloc/alloc-addr");
}

#endif // STDLIB_C