mod error;

use crate::print;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use tinywasm::{Extern, FuncContext, MemoryStringExt};

#[derive(Clone, Copy)]
struct Allocation {
    ptr: usize,
    size: usize,
}

#[derive(Clone)]
struct AllocCommandReturn(Vec<Allocation>, usize);

fn malloc(allocations: Vec<Allocation>, size: usize) -> Result<AllocCommandReturn, String> {
    // try and find room
    let mut ptr = 0;
    for allocation in allocations.iter() {
        if allocation.ptr >= ptr && allocation.ptr - ptr >= size {
            break;
        }
        ptr = allocation.ptr + allocation.size;
    }
    // if we didn't find room, allocate at the end
    if ptr == 0 {
        if let Some(last) = allocations.last() {
            ptr = last.ptr + last.size;
        }
    }
    let mut new_allocations = allocations.clone();
    new_allocations.push(Allocation { ptr, size });
    Ok(AllocCommandReturn(new_allocations, ptr))
}

fn free(allocations: Vec<Allocation>, ptr: usize) -> Result<Vec<Allocation>, String> {
    let mut new_allocations = Vec::new();
    for allocation in allocations.iter() {
        if allocation.ptr == ptr {
            continue;
        }
        new_allocations.push(*allocation);
    }
    Ok(new_allocations)
}

static mut ALLOCATIONS: Vec<Allocation> = Vec::new();
static mut ALLOC_SIZE: u32 = 0;
static mut REALLOC_PTR: u32 = 0;

pub fn run_from_bytes(bytes: &[u8]) -> Result<(), String> {
    let module = match tinywasm::Module::parse_bytes(bytes) {
        Ok(module) => module,
        Err(err) => return Err(err.to_string()),
    };
    let mut store = tinywasm::Store::default();
    let mut imports = tinywasm::Imports::new();

    // read from a stream
    match imports.define(
        "env",
        "read",
        Extern::typed_func(|mut context: FuncContext<'_>, data: i32| {
            // read from a stream
            // data is the pointer to the stream name
            // we return a pointer to the data
            let mem = context.exported_memory("memory");
            match mem {
                Ok(mem) => {
                    let name = mem.load_cstring_until_nul(data as usize, 100);
                    match name {
                        Ok(name) => {
                            let name = name.to_str().unwrap();
                            match name {
                                "/dev/allocator/alloc" => {
                                    let size = unsafe { ALLOC_SIZE };
                                    let result =
                                        malloc(unsafe { ALLOCATIONS.clone() }, size as usize);
                                    match result {
                                        Ok(AllocCommandReturn(new_allocations, ptr)) => {
                                            unsafe {
                                                ALLOCATIONS = new_allocations;
                                            }
                                            Ok(ptr as i32)
                                        }
                                        Err(_) => return Ok(error::Error::OutOfMemory as i32),
                                    }
                                }
                                "/dev/allocator/realloc" => {
                                    let ptr = unsafe { REALLOC_PTR };
                                    let size = unsafe { ALLOC_SIZE };
                                    let result = free(unsafe { ALLOCATIONS.clone() }, ptr as usize);
                                    match result {
                                        Ok(new_allocations) => {
                                            unsafe {
                                                ALLOCATIONS = new_allocations;
                                            }
                                            let result = malloc(
                                                unsafe { ALLOCATIONS.clone() },
                                                size as usize,
                                            );
                                            match result {
                                                Ok(AllocCommandReturn(new_allocations, ptr)) => {
                                                    unsafe {
                                                        ALLOCATIONS = new_allocations;
                                                    }
                                                    Ok(ptr as i32)
                                                }
                                                Err(_) => {
                                                    return Ok(error::Error::OutOfMemory as i32)
                                                }
                                            }
                                        }
                                        Err(_) => return Ok(error::Error::NotFound as i32),
                                    }
                                }
                                _ => Ok(error::Error::NotFound as i32),
                            }
                        }
                        Err(_) => return Ok(error::Error::MalformedRequest as i32),
                    }
                }
                Err(_) => return Ok(error::Error::MalformedRequest as i32),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // write to a stream
    match imports.define(
        "env",
        "write",
        Extern::typed_func(|mut context: FuncContext<'_>, data: (i32, i32, i32)| {
            // write to a stream
            // data.0 is the pointer to the stream name
            // data.1 is a pointer to the beginning of the data
            // data.2 is the length of the data
            let mem = context.exported_memory("memory");
            match mem {
                Ok(mem) => {
                    let name = mem.load_cstring_until_nul(data.0 as usize, 100);
                    match name {
                        Ok(name) => {
                            let name = name.to_str().unwrap();
                            let bytes = mem.load(data.1 as usize, data.2 as usize);
                            match bytes {
                                Ok(bytes) => {
                                    let bytes = bytes.to_vec();
                                    match name {
                                        "/dev/stdout" => {
                                            let mut string = String::new();
                                            for byte in bytes.iter() {
                                                string.push(*byte as char);
                                            }
                                            print!("{}", string);
                                            Ok(0)
                                        }
                                        "/dev/allocator/alloc-size" => {
                                            // turn bytes into u32
                                            let mut size = 0;
                                            for byte in bytes.iter() {
                                                size = size << 8;
                                                size += *byte as u32;
                                            }
                                            unsafe {
                                                ALLOC_SIZE = size;
                                            }
                                            Ok(0)
                                        }
                                        "/dev/allocator/realloc-addr" => {
                                            // turn bytes into u32
                                            let mut ptr = 0;
                                            for byte in bytes.iter() {
                                                ptr = ptr << 8;
                                                ptr += *byte as u32;
                                            }
                                            unsafe {
                                                REALLOC_PTR = ptr;
                                            }
                                            Ok(0)
                                        }
                                        "/dev/allocator/free" => {
                                            // turn bytes into u32
                                            let mut ptr = 0;
                                            for byte in bytes.iter() {
                                                ptr = ptr << 8;
                                                ptr += *byte as u32;
                                            }
                                            let result = free(unsafe { ALLOCATIONS.clone() }, ptr as usize);
                                            match result {
                                                Ok(new_allocations) => {
                                                    unsafe {
                                                        ALLOCATIONS = new_allocations;
                                                    }
                                                    Ok(0)
                                                }
                                                Err(_) => return Ok(error::Error::NotFound as i32),
                                            }
                                        }
                                        _ => Ok(error::Error::NotFound as i32),
                                    }
                                }
                                Err(_) => return Ok(error::Error::MalformedRequest as i32),
                            }
                        }
                        Err(_) => return Ok(error::Error::MalformedRequest as i32),
                    }
                }
                Err(_) => return Ok(error::Error::MalformedRequest as i32),
            }
        }),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    }

    // instantiating the module will run the start function
    let instance = match module.instantiate(&mut store, Some(imports)) {
        Ok(instance) => instance,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    return Ok(());
}
