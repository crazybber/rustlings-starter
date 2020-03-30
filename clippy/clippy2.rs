// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {  //use destructure to retrive its inner value
        res += x;
    }
    println!("{}", res);
}
