// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_this(3);
}

fn call_this(index:u32) {
    for i in 0..index {
        println!("Loop! number {}", i + 1);

    }
}

