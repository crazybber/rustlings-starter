// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; //here need a ";"


    // this is a capture expression ,following are allowed.
    /*
    item: an item, like a function, struct, module, etc.
    block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
    stmt: a statement
    pat: a pattern
    expr: an expression
    ty: a type
    ident: an identifier
    path: a path (e.g. , , , …)foo::std::mem::replacetransmute::<_, int>
    meta: a meta item; the things that go inside and attributes#[...]#![...]
    tt: a single token tree
    */
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
