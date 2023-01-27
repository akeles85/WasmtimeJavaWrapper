#include "WasmtimeWrapper.h"

extern "C" {
extern int wasmtime_multiply(int32_t, int32_t);
}

JNIEXPORT jint JNICALL Java_WasmtimeWrapper_multiply
  (JNIEnv *, jobject, jint value1, jint value2)
{
    return wasmtime_multiply(value1,value2);    
}
