#[macro_use]
extern crate criterion;
extern crate memo_int_sqrt;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("control inverse squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box((i as f32).sqrt().recip());
            }
        });
    });
    c.bench_function("memo_int_sqrt u8 inverse squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u8::f32::inv_sqrt(i as u8));
            }
        });
    });
    c.bench_function("memo_int_sqrt u16 inverse squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u16::f32::inv_sqrt(i as u16));
            }
        });
    });
    c.bench_function("memo_int_sqrt u32 inverse squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u32::f32::inv_sqrt(i as u32));
            }
        });
    });
    c.bench_function("control squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box((i as f32).sqrt());
            }
        });
    });
    c.bench_function("memo_int_sqrt u8 squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u8::f32::sqrt(i as u8));
            }
        });
    });
    c.bench_function("memo_int_sqrt u16 squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u16::f32::sqrt(i as u16));
            }
        });
    });
    c.bench_function("memo_int_sqrt u32 squareroot f32", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u32::f32::sqrt(i as u32));
            }
        });
    });

    c.bench_function("control inverse squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box((i as f64).sqrt().recip());
            }
        });
    });
    c.bench_function("memo_int_sqrt u8 inverse squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u8::f64::inv_sqrt(i as u8));
            }
        });
    });
    c.bench_function("memo_int_sqrt u16 inverse squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u16::f64::inv_sqrt(i as u16));
            }
        });
    });
    c.bench_function("memo_int_sqrt u32 inverse squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u32::f64::inv_sqrt(i as u32));
            }
        });
    });
    c.bench_function("control squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box((i as f64).sqrt());
            }
        });
    });
    c.bench_function("memo_int_sqrt u8 squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u8::f64::sqrt(i as u8));
            }
        });
    });
    c.bench_function("memo_int_sqrt u16 squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u16::f64::sqrt(i as u16));
            }
        });
    });
    c.bench_function("memo_int_sqrt u32 squareroot f64", |b| {
        b.iter(|| {
            for i in 0..256 {
                criterion::black_box(memo_int_sqrt::u32::f64::sqrt(i as u32));
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
