wit_bindgen_rust::export!("../wits/sql.wit");

struct Sql;

impl sql::Sql for Sql {
    fn sqlmethod(s8t: i8, s16t: i16, s32t: i32, s64t: i64, float32t: f32, float64t: f64, chart: char, stringt : String) -> (i8, i16, i32, i64, f32, f64, char, String) {
        (s8t, s16t, s32t, s64t, float32t, float64t, chart, stringt)
    }
    
}

