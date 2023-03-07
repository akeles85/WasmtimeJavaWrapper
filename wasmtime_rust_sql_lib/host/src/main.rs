#![allow(unused_variables, dead_code)]

wit_bindgen_wasmtime::import!("../wits/sql.wit");

use anyhow::Result;
use wit_bindgen_wasmtime::wasmtime::{self, Config, Engine, Instance, Linker, Module, Store};

fn main() {
    let path = if cfg!(not(debug_assertions)) {
        "./target/wasm32-wasi/release/guest.wasm"
    } else {
        "./target/wasm32-wasi/debug/guest.wasm"
    };
    println!("\n\n\n\n\n");
    println!("Running from the guest wasm");
    run(path, "Michael");
}

fn run(path: &str, name: &str) {
    use sql::{Sql, SqlData};
    type SqlStore = Store<Context<SqlData, SqlData>>;

    let funcs = instantiate(path, |store: &mut SqlStore, module, linker| {
        Sql::instantiate(store, module, linker, |cx| &mut cx.exports)
    });

    let i8_t: i8;
    let i16_t : i16;
    let i32_t : i32;
    let i64_t : i64;
    let f32_t : f32;
    let f64_t : f64;
    let c_t : char;
    let string_T : String;     

    if let Ok((exports, mut store)) = funcs {
        match exports.sqlmethod(&mut store, 1,2,3,4,1.1,2.2, 'c', "string") {
            Ok((i8_t, i16_t, i32_t, i64_t, f32_t, f64_t, c_t, string_t)) => println!("{},{},{},{},{},{},{},{}", i8_t, i16_t, i32_t, i64_t, f32_t, f64_t, c_t, string_t   ),
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
