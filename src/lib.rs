pub mod tables {
    include!(concat!(env!("OUT_DIR"), "/tables.rs"));
}

pub mod u8 {
    pub mod f32 {
        pub fn sqrt(x: u8) -> f32 {
            crate::tables::SQRT_F32[x as usize]
        }
        pub fn inv_sqrt(x: u8) -> f32 {
            crate::tables::INV_SQRT_F32[x as usize]
        }
    }
    pub mod f64 {
        pub fn sqrt(x: u8) -> f64 {
            crate::tables::SQRT_F64[x as usize]
        }
        pub fn inv_sqrt(x: u8) -> f64 {
            crate::tables::INV_SQRT_F64[x as usize]
        }
    }
}

macro_rules! unsigned_inner {
    ($in:ident, $out:ident) => {
        pub mod $out {
            pub fn sqrt(x: $in) -> $out {
                if x < 256 {
                    crate::u8::$out::sqrt(x as u8)
                } else {
                    (x as $out).sqrt()
                }
            }
            pub fn inv_sqrt(x: $in) -> $out {
                if x < 256 {
                    crate::u8::$out::inv_sqrt(x as u8)
                } else {
                    1. / (x as $out).sqrt()
                }
            }
        }
    };
}
macro_rules! unsigned {
    ($in:ident) => {
        pub mod $in {
            unsigned_inner!($in, f32);
            unsigned_inner!($in, f64);
        }
    };
}
unsigned!(u16);
unsigned!(u32);
unsigned!(u64);
unsigned!(u128);

#[cfg(test)]
mod test {
    macro_rules! test {
        ($in:ident, $count:expr) => {
            #[test]
            fn $in() {
                for x in 0..$count {
                    assert_eq!((x as f32).sqrt(), crate::$in::f32::sqrt(x as $in));
                    assert_eq!(1. / (x as f32).sqrt(), crate::$in::f32::inv_sqrt(x as $in));
                    assert_eq!((x as f64).sqrt(), crate::$in::f64::sqrt(x as $in));
                    assert_eq!(1. / (x as f64).sqrt(), crate::$in::f64::inv_sqrt(x as $in));
                }
            }
        };
    }
    test!(u8, 255);
    test!(u16, 1024);
    test!(u32, 1024);
    test!(u64, 1024);
    test!(u128, 1024);
}
