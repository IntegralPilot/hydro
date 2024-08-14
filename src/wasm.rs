mod error;

use alloc::string::{ToString, String};
use tinywasm::{FuncContext, MemoryStringExt, Extern};
use crate::print;

pub fn run_from_bytes(bytes: &[u8]) -> Result<(), String> {
    let module = match tinywasm::Module::parse_bytes(bytes) {
        Ok(module) => module,
        Err(err) => return Err(err.to_string()),
    };
    let mut store = tinywasm::Store::default();
    let mut imports = tinywasm::Imports::new();


    // write to a stream
    match imports.define("env", "write", Extern::typed_func(|mut context: FuncContext<'_>, data: (i32, i32, i32)| {
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
                                    _ => Ok(error::Error::NotFound as i32),
                                }
                            }
                            Err(_) => return Ok(error::Error::MalformedRequest as i32),
                        }
                    },
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