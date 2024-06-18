// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(None);
}

fn call_me(num: Option<u32>) {
    for i in 0..num.unwrap_or(3) {
        println!("Ring! Call number {}", i + 1);
    }
}
