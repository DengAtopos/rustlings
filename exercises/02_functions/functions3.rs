// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(2);
}

fn call_me(num: u32) {//u32是无符号类型,没有负数整型
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
