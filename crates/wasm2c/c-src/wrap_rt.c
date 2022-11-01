#include "wasm-rt.h"
#include <stdint.h>
#include <stdlib.h>

uint32_t wasm_rt_register_func_type_wrap(uint32_t params, uint32_t results,
                                         wasm_rt_type_t *types);

uint32_t wasm_rt_register_func_type(uint32_t params, uint32_t results, ...) {
  wasm_rt_type_t *temp_buf = alloca(params + results);
  return wasm_rt_register_func_type_wrap(params, results, temp_buf);
}
