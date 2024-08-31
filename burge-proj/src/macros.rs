#[macro_export]
macro_rules! try_vec2 {
    ($value:tt) => {
        if let serde_json::Value::Array(vec2) = $value {
            let mut ret = [0.0,0.0];
            for i in 0..2 {
                if let Value::Number(num) = &vec2[i] {
                    if let Some(num) = num.as_f64() {
                        ret[i] = num as f32;
                    }
                }
            }
            Some(ret)
        } else {
            None
        }
    };
}