use proc_macro2::Literal;

pub(crate) trait Convert {
    fn to_int3(&self) -> [Literal; 3];
    fn to_int4(&self) -> [Literal; 4];
    fn to_float3(&self) -> [Literal; 3];
    fn to_float4(&self) -> [Literal; 4];
}


impl Convert for [i64; 4] {
    fn to_int3(&self) -> [Literal; 3] {
        [
            Literal::i64_unsuffixed(self[0]),
            Literal::i64_unsuffixed(self[1]),
            Literal::i64_unsuffixed(self[2]),
        ]
    }

    fn to_int4(&self) -> [Literal; 4] {
        [
            Literal::i64_unsuffixed(self[0]),
            Literal::i64_unsuffixed(self[1]),
            Literal::i64_unsuffixed(self[2]),
            Literal::i64_unsuffixed(self[3]),
        ]
    }

    fn to_float3(&self) -> [Literal; 3] {
        if self[0..3].iter().all(|x|(0..=1).contains(x)){
            [
                Literal::f64_unsuffixed(self[0] as f64),
                Literal::f64_unsuffixed(self[1] as f64),
                Literal::f64_unsuffixed(self[2] as f64),
            ]
        } else {
            [
                Literal::f64_unsuffixed(self[0] as f64 / 255.0),
                Literal::f64_unsuffixed(self[1] as f64 / 255.0),
                Literal::f64_unsuffixed(self[2] as f64 / 255.0),
            ]
        }
    }

    fn to_float4(&self) -> [Literal; 4] {
        if self.iter().all(|x|(0..=1).contains(x)){
            [
                Literal::f64_unsuffixed(self[0] as f64),
                Literal::f64_unsuffixed(self[1] as f64),
                Literal::f64_unsuffixed(self[2] as f64),
                Literal::f64_unsuffixed(self[3] as f64),
            ]
        } else {
            [
                Literal::f64_unsuffixed(self[0] as f64 / 255.0),
                Literal::f64_unsuffixed(self[1] as f64 / 255.0),
                Literal::f64_unsuffixed(self[2] as f64 / 255.0),
                Literal::f64_unsuffixed(self[3] as f64 / 255.0),
            ]
        }
    }
}


impl Convert for [f64; 4] {
    fn to_int3(&self) -> [Literal; 3] {
        [
            Literal::i64_unsuffixed((self[0] * 255.0) as i64),
            Literal::i64_unsuffixed((self[1] * 255.0) as i64),
            Literal::i64_unsuffixed((self[2] * 255.0) as i64),
        ]
    }

    fn to_int4(&self) -> [Literal; 4] {
        [
            Literal::i64_unsuffixed((self[0] * 255.0) as i64),
            Literal::i64_unsuffixed((self[1] * 255.0) as i64),
            Literal::i64_unsuffixed((self[2] * 255.0) as i64),
            Literal::i64_unsuffixed((self[3] * 255.0) as i64),
        ]
    }

    fn to_float3(&self) -> [Literal; 3] {
        [
            Literal::f64_unsuffixed(self[0]),
            Literal::f64_unsuffixed(self[1]),
            Literal::f64_unsuffixed(self[2]),
        ]
    }

    fn to_float4(&self) -> [Literal; 4] {
        [
            Literal::f64_unsuffixed(self[0]),
            Literal::f64_unsuffixed(self[1]),
            Literal::f64_unsuffixed(self[2]),
            Literal::f64_unsuffixed(self[3]),
        ]
    }
}
