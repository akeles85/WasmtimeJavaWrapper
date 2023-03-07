#![allow(unused_variables, dead_code)]
wit_bindgen_wasmtime::import!("../wits/sql.wit");

use anyhow::Result;
use wit_bindgen_wasmtime::wasmtime::{self, Config, Engine, Instance, Linker, Module, Store};
use jni::JNIEnv;
use jni::objects::{JClass, JString};


fn run(p_s8: i8, p_s16: i16, p_s32: i32, p_s64: i64, p_float32: f32, p_float64: f64, p_char: char, p_string: String) {
    let path = "sql_udf.wasm";
    let string_t = "test";
    use sql::{Sql, SqlData};
    type SqlStore = Store<Context<SqlData, SqlData>>;

    let funcs = instantiate(path, |store: &mut SqlStore, module, linker| {
        Sql::instantiate(store, module, linker, |cx| &mut cx.exports)
    });
     
    if let Ok((exports, mut store)) = funcs {
        match exports.sqlmethod(&mut store, p_s8, p_s16, p_s32, p_s64, p_float32, p_float64, p_char, string_t) {
            Ok((i8_t, i16_t, i32_t, i64_t, f32_t, f64_t, c_t, string_t)) => println!("{},{},{},{},{},{},{},{}", p_s8, p_s16, p_s32, p_s64, p_float32, p_float64, p_char, string_t   ),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("no instantiate");
    }
}


fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}

fn instantiate<'a, I: Default, E: Default, T>(
    wasm: &str,
    mk_exports: impl FnOnce(
        &mut Store<Context<I, E>>,
        &Module,
        &mut Linker<Context<I, E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<I, E>>)> {
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Disable);

    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    //add_imports(&mut linker)?;
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<I, E>| &mut cx.wasi)?;

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: I::default(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;
    Ok((exports, store))
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_WasmtimeWrapper_sqlTypesMethod(env: JNIEnv,
                                             class: JClass,
                                             p_s8: i8, p_s16: i16, p_s32: i32, p_s64: i64, p_float32: f32, p_float64: f64, p_char: char, p_jstring :JString){
    
    let p_rust_string: String = env.get_string(p_jstring).unwrap().into();                                                
    run( p_s8, p_s16, p_s32, p_s64, p_float32, p_float64, p_char, p_rust_string );
}