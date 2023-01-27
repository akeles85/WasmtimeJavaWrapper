
public class WasmtimeWrapper {

    static {
        System.loadLibrary("wasmtime_wrapper_lib");
    }
     
    public static void main(String[] args) {
        WasmtimeWrapper wrapper = new WasmtimeWrapper();
        int result = wrapper.multiply(3,5);
        System.out.println("Result: " + result);
    }
      
    private native int multiply(int value1, int value2);
}