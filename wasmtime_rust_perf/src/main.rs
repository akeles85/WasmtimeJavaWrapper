use anyhow::Result;
use wasmtime::*;


fn multiply_native(value1: f32, value2: f32) -> Result<f32>{
    return Ok(value1.cos()*value2.sin());
}


fn perf_func(x: f32, y: f32) -> Result<f32> {
    use std::time::Instant;

    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Disable);    
    config.debug_info(false);
    let engine = Engine::new(&config)?;        
    let mut store = Store::new(&engine, ());
    
    let module = Module::from_file(store.engine(), "mult.wasm")?;
    let instance = Instance::new(&mut store, &module, &[])?;
    
    let wat_multiply = instance.get_typed_func::<(f32, f32), f32, _>(&mut store, "multiply")?;

    let mut now_wasm = Instant::now();
    let mut result = 0.0;
    let mut total_sum_wasm = 0.0;
    for n in 0..1 {
        match wat_multiply.call(&mut store, (x, y)){
            Ok(result) => total_sum_wasm += result,
            Err(e) => println!("Error: {}", e),
        }
    }
    let mut elapsed_wasm = now_wasm.elapsed();    

    let now_native = Instant::now();

    let mut total_sum_native = 0.0;
    for n in 0..10_000_000 {        
        match multiply_native(x, y){
            Ok(result) => total_sum_native += result,
            Err(e) => println!("Error: {}", e),
        }
    }
    let elapsed_native = now_native.elapsed();     

    println!("Results: Wasm {}, Native: {}", total_sum_wasm, total_sum_native );
    println!("Elapsed Time: Wasm {:.2?}, Native: {:.2?}", elapsed_wasm, elapsed_native );

    return Ok(result);
}

fn main(){    
    perf_func(3.0,4.0);
}
