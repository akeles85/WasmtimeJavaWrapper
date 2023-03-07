

public class WasmtimeWrapper {

    static {
        System.loadLibrary("wasmtime_wrapper_sql_lib");
    }
     
    public static void main(String[] args) {
        WasmtimeWrapper wrapper = new WasmtimeWrapper();        
        
        byte i8 = 1;
        short i16 = 2;
        int i32 = 3;
        long i64 = 4;
        float f32 = 1.1f;
        double f64 = 2.2;
        char x = 'c';
        String s = "test";
                
        wrapper.sqlTypesMethod(i8, i16, i32, i64, f32, f64, x, s);
    }
      
    private native void sqlTypesMethod(byte p1, short p2, int p3, long p4, float p5, double p6, char p7, String p8);
}

class SqlResult{
    public byte i8;
    public short i16;
    public int i32;
    public long i64;
    public float f32;
    public double f64;
    public char x;
    public String s;
};