# memo_int_sqrt

Definitely run the benchmarks before using this. In my benchmarking, I found that for `f32`s, `.sqrt().recip()` is often
faster than using the lookup table. For `f64`s, the lookup table is faster for inverse squareroot, but not for squareroot.
