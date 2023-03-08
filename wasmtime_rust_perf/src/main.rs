use anyhow::Result;
use wasmtime::*;

fn multiply_native(value1: i32, value2: i32) -> Result<i32>{
    return Ok(value1*value2);
}

fn perf_func(x: i32, y: i32) -> Result<i32> {
    use std::time::Instant;
    let mut store = Store::<()>::default();
    
    let module = Module::from_file(store.engine(), "multiply.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;
    
    let wat_multiply = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "multiply")?;

    let now_wasm = Instant::now();
    let mut result = 0;
    let mut total_sum_wasm = 0;
    for n in 0..10_000_000 {
        match wat_multiply.call(&mut store, (x, y)){
            Ok(result) => total_sum_wasm += result,
            Err(e) => println!("Error: {}", e),
        }
    }
    let elapsed_wasm = now_wasm.elapsed(); 

    let now_native = Instant::now();

    let mut total_sum_native = 0;
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
    perf_func(3,4);
}
