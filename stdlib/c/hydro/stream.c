#ifndef _HYDRO_STREAM_C
#define _HYDRO_STREAM_C

int autowrite(char* stream_name, int* buf) {
    return write(stream_name, buf, strlen((char*)buf));
}

#endif // HYDRO_STREAM_C