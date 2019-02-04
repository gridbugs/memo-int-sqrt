use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn process<F: Fn(usize) -> String>(count: usize, f: F) -> String {
    (0..count)
        .map(|x| format!("    {},\n", f(x)))
        .collect::<Vec<_>>()
        .join("")
}

fn part<F: Fn(usize) -> String>(name: &str, typ: &str, count: usize, f: F) -> Vec<u8> {
    format!(
        "pub const {}: [{}; {}] = [\n{}];\n",
        name,
        typ,
        count,
        process(count, f)
    )
    .into_bytes()
}

fn generate(path: &Path) -> io::Result<()> {
    let mut f = File::create(&path)?;
    f.write_all(&part("SQRT_F32", "f32", 256, |x| {
        format!("{:.64}", (x as f32).sqrt())
    }))?;
    f.write_all(&part("SQRT_F64", "f64", 256, |x| {
        format!("{:.64}", (x as f64).sqrt())
    }))?;
    f.write_all(&part("INV_SQRT_F32", "f32", 256, |x| match x {
        0 => "::std::f32::INFINITY".to_string(),
        non_zero => format!("{:.64}", 1. / (non_zero as f32).sqrt()),
    }))?;
    f.write_all(&part("INV_SQRT_F64", "f64", 256, |x| match x {
        0 => "::std::f64::INFINITY".to_string(),
        non_zero => format!("{:.64}", 1. / (non_zero as f64).sqrt()),
    }))?;
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tables.rs");
    generate(&dest_path).unwrap();
}
