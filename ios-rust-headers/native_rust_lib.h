#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

int32_t rust_add(int32_t a, int32_t b);

// If you use the string helper:
const uint8_t* hello(uint32_t* out_len);

#ifdef __cplusplus
} // extern "C"
#endif
