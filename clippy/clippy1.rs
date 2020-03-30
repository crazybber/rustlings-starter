// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// Execute `rustlings hint clippy1` for hints :)


fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;

//if y != x {} // where both are floats, so do not use this
//https://rust-lang.github.io/rust-clippy/master/#float_cmp
//Floating point calculations are usually imprecise, so asking if two values are exactly equal is asking for trouble
    if (y - x).abs() < 1e-6 {
        println!("Success!");
    }
}
