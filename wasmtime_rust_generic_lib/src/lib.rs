use jni::JNIEnv;
use jni::objects::{JClass, JString};
use anyhow::Result;
use wasmtime::*;

fn dynamic_method( method: String, params: Vec<Val>, result: &mut Vec<Val>) {
    let mut store = Store::<()>::default();

    let method_file :String = method.clone() + ".wat";
    let module = Module::from_file(store.engine(), &method_file[..]).expect("cannot find wat file");
    let instance = Instance::new(&mut store, &module, &[]).expect("cannot create instance");

    let wat_multiply = instance.get_func(&mut store, &method[..] ).expect("export wasn't a function");
    
    //result.as_mut_slice()
    wat_multiply.call(&mut store, params.as_slice(), result.as_mut_slice()).expect("function call failed");    
    
    return;
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_WasmtimeWrapper_multiply(env: JNIEnv,
                                             class: JClass,
                                             value1: i32, value2: i32)
                                             -> i32 {
    let mut params: Vec<Val> = Vec::new();    
    params.push(Val::I32(3));
    params.push(Val::I32(4));   
    
    let mut result: Vec<Val> = Vec::new();    
    result.push(Val::I32(0));


    dynamic_method( "multiply".to_string(), params, &mut result);

    // Pattern match to retrieve the value
    match result[0].i32() {
        // The division was valid
        Some(x) => return x,
        // The division was invalid
        None    => return 0,
    }
}