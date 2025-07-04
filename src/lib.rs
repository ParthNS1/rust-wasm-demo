use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn heavy_fib_task(max_n: u32) -> u64 {
    let mut sum = 0;
    for n in 30..=max_n {
        sum += fib(n);
    }
    sum
}

fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
