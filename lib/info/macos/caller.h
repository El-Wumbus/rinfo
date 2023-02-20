#ifndef INFO_MACOS_CALLER
#define INFO_MACOS_CALLER

#ifndef __APPLE__
#include <stdint.h>
#else
typedef unsigned long long uint64_t;
#endif

typedef __SIZE_TYPE__ size_t;


int macos_get_caller(size_t pid, char* buffer, size_t buffer_len);
#endif