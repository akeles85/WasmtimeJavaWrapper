
# Introduction

wasmtime is chosen as web assembly runtime in this project.

We can prepare wasm files from mainly Rust, C, Assembly Script and directly via Web Assembly Text (.wat) file to use with wasmtime.

We can call these files from different languages: Rust, C, Python, .Net, Go and Ruby.

In this project, both wasm and wat file is used as example with Rust and C programming languages.
We can call the methods with dynamic parameter and types parameters. Both of the usage is given in examples. ( wasmtime_rust_lib, wasmtime_rust_generic_lib)

'''
    let mut params: Vec<Val> = Vec::new();    
    params.push(Val::I32(3));
    params.push(Val::I32(4));   
    
    let mut result: Vec<Val> = Vec::new();    
    result.push(Val::I32(0));
'''    


Web Assembly V1
    Just supports signed 32, signed 64, float 32, float 64

Web Assembly V2
    Supports vector, reference, value and function types in addition to V1.

WebAssembly is just an assembly language and you should design your data structures using this types.
Thanks to the binding generators, we do not need to do this manually in each project.
Wit-bindgen, fp-bindgen, wasm-bindgen is some of the binding generators. In this repository, I used the wit-bindgen to use the following types parameters to wasm method.
I mostly relied on wit-bindgen-example (Given at references) to use these types. 
Unfortunetaly, this library does not have a dynamic call method.

signed 8 byte
signed 16 byte
signed 32 byte
signed 64 byte
float 32
float 64
char
string

References:

https://github.com/bytecodealliance/wit-bindgen/tree/v0.1.0
https://github.com/fiberplane/fp-bindgen
https://blog.scottlogic.com/2022/04/16/wasm-faas.html
https://docs.wasmtime.dev/introduction.html
https://petermalmgren.com/serverside-wasm-data/
https://github.com/masmullin2000/wit-bindgen-example


# Usage

1. First clone wasmtime repository (https://github.com/bytecodealliance/wasmtime)
2. Build the wasmtime repository with cargo build
3. Clone the wit-bindgen repository(v0.1.0) https://github.com/bytecodealliance/wit-bindgen

## For C Wrapper:
    1. Then run the cmake file in wasmtime_cpp_lib to build the shared library that will be used for java 
    2. Run WasmTimeWrapper java file to run the multiply example.

## For Rust Wrapper.
    1. Go to wasmtime_rust_lib directory
    2. Build the shared library with "cargo build --release" command.
    3. Copy the generated target/release/libwasmtime_wrapper_lib.dylib to the directory of Java file.

## For Rust Performance Test.
    1. Go to wasmtime_rust_perf directory
    2. Build the executable with "cargo build --release" command.
    3. Copy wasm/multiply.wasm to wasmtime_rust_perf directory
    4. Run the following command to start perf test
        ./target/release/perf_test

    Example output:
    Elapsed Time: Wasm 3.03s, Native: 93.27ms

## For Rust SQL Types.
    1. Go to the wasmtime_rust_sql_lib directory
    2. Build the libraries with "make release" command.
    3. Copy the generated target/release/libwasmtime_wrapper_sql_lib.dylib to the wasm_sql directory
    4. Copy the generated target/wasm32-wasi/release/sql_udf.wasm to the wasm_sql directory
    5. Run WasmtimeWrapper.java file to see the output of types printed in wasm.





