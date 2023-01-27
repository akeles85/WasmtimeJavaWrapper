use jni::JNIEnv;
use jni::objects::{JClass, JString};
use anyhow::Result;
use wasmtime::*;

fn multiply(x: i32, y: i32) -> Result<i32> {
    let mut store = Store::<()>::default();
    let module = Module::from_file(store.engine(), "multiply.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    let wat_multiply = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "multiply")?;

    let result = wat_multiply.call(&mut store, (x, y))?;
    
    return Ok(result);
}

fn multiply_op(x: i32, y: i32) -> i32 {
    match multiply(x,y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => ratio
    }
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_WasmtimeWrapper_multiply(env: JNIEnv,
                                             class: JClass,
                                             value1: i32, value2: i32)
                                             -> i32 {
    return multiply_op(value1, value2)    
}