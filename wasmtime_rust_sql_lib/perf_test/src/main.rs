#![allow(unused_variables, dead_code)]
wit_bindgen_wasmtime::import!("../wits/sql.wit");

use anyhow::Result;
use wit_bindgen_wasmtime::wasmtime::{self, Config, Engine, Instance, Linker, Module, Store};
use jni::JNIEnv;
use jni::objects::{JClass, JString};


fn sqlmethod_native(s8t: i8, s16t: i16, s32t: i32, s64t: i64, float32t: f32, float64t: f64, chart: char, stringt : String) -> (i8, i16, i32, i64, f32, f64, char, String, i32) {
    let result: i32;
    let rc = format!("hello {}", stringt);
    //A simple calculation
    result = (s32t * 2) + 2;
    (s8t, s16t, s32t, s64t, float32t, float64t, chart, stringt, result)
}

fn add_native(value1: i32, value2: i32) -> (i32){
    return value1+value2;
}

fn perf_test() {
    use std::time::Instant;
    let path = "sql_udf.wasm";
    use sql::{Sql, SqlData};
    type SqlStore = Store<Context<SqlData, SqlData>>;

    let funcs = instantiate(path, |store: &mut SqlStore, module, linker| {
        Sql::instantiate(store, module, linker, |cx| &mut cx.exports)
    });
     
    let mut total_sum_wasm = 0;
    let now_wasm = Instant::now();
           
    if let Ok((exports, mut store)) = funcs {                
        for n in 0..10_000_000 {
            match exports.add(&mut store, n, n+1) {
                Ok(tmp_result) => total_sum_wasm += tmp_result,
                Err(e) => println!("Error: {}", e),
            }
        }
    } else {
        println!("no instantiate");
    }

    let elapsed_wasm = now_wasm.elapsed();  

    let now_native = Instant::now();

    let mut total_sum_native = 0;
    for n in 0..10_000_000 {
        let mut tmp_result = 0;
        tmp_result = add_native(n, n+1);
        total_sum_native += tmp_result;
    }
    let elapsed_native = now_native.elapsed(); 

    println!("{};{};{:.2?};{:.2?}", total_sum_wasm,total_sum_native, elapsed_wasm, elapsed_native);
}

fn main(){
    perf_test();
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
