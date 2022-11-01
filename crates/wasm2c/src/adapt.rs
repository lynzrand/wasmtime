use wasmtime::{component::Val, Caller, Trap};

use crate::State;

/*
void wasm_rt_init(void);
bool wasm_rt_is_initialized(void);
void wasm_rt_free(void);
void wasm_rt_trap(wasm_rt_trap_t) __attribute__((noreturn));
const char* wasm_rt_strerror(wasm_rt_trap_t trap);
uint32_t wasm_rt_register_func_type(uint32_t params, uint32_t results, ...);
void wasm_rt_allocate_memory(wasm_rt_memory_t*, uint32_t initial_pages, uint32_t max_pages);
uint32_t wasm_rt_grow_memory(wasm_rt_memory_t*, uint32_t pages);
void wasm_rt_free_memory(wasm_rt_memory_t*);
void wasm_rt_allocate_funcref_table(wasm_rt_table_t*, uint32_t elements, uint32_t max_elements);
void wasm_rt_allocate_externref_table(wasm_rt_externref_table_t*, uint32_t elements, uint32_t max_elements);
void wasm_rt_free_funcref_table(wasm_rt_table_t*);
void wasm_rt_free_externref_table(wasm_rt_table_t*);
 */

#[repr(C)]
enum CTrap {
    None,
    OutOfBounds,
    IntOverflow,
    DivByZero,
    InvalidConversion,
    Unreachable,
    CallIndirect,
    Exhaustion,
}

#[repr(C)]
enum CType {
    I32,
    I64,
    F32,
    F64,
}

/// Functions that need to be implemented by the host
mod implementation {
    use std::panic::panic_any;

    use super::{CTrap, CType};

    #[no_mangle]
    extern "C" fn wasm_rt_init() {}

    #[no_mangle]
    extern "C" fn wasm_rt_is_initialized() -> bool {
        true
    }

    #[no_mangle]
    extern "C" fn wasm_rt_free() {}

    /// A trap has occurred in the wasm2c code. We have nothing to do other than
    /// panicking and unwind the stacks since we can't go through all those C
    /// frames.
    #[no_mangle]
    extern "C" fn wasm_rt_trap(trap: CTrap) -> ! {
        // Man, panicking is the only thing we can do here
        panic_any(trap)
    }

    /// Returns the string representation of the error.
    ///
    /// `const char* wasm_rt_strerror(wasm_rt_trap_t trap)`
    #[no_mangle]
    extern "C" fn wasm_rt_strerror(trap: CTrap) -> *const u8 {
        let err = strerror(trap);
        err.as_ptr()
    }

    #[no_mangle]
    extern "C" fn wasm_rt_register_func_type_wrap(
        n_params: u32,
        n_result: u32,
        vals: *const CType,
    ) {
    }
}

/// Returns the description of the trap.
///
/// # Caution
///
/// Every string returned in this function has a null byte after it to make C happy.
/// Strip this byte.
fn strerror(trap: CTrap) -> &str {
    // add a null byte in the end to make C happy
    let err = match trap {
        CTrap::None => "No error\0",
        CTrap::OutOfBounds => "Memory out of bounds\0",
        CTrap::IntOverflow => "Integer overflow\0",
        CTrap::DivByZero => "Division by zero\0",
        CTrap::InvalidConversion => "Invalid conversion\0",
        CTrap::Unreachable => "Unreachable code\0",
        CTrap::CallIndirect => "Invalid indirect calling\0",
        CTrap::Exhaustion => "Memory exhaustion\0",
    };
    err
}

// impl Fn(Caller<'_, T>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static
fn call_wasm2c<T: AsMut<State>>(
    caller: Caller<'_, T>,
    params: &[Val],
    res: &mut [Val],
) -> Result<(), Trap> {
    let state = caller.data_mut().as_mut();

    Ok(())
}
