include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
    unsafe { println!("1+1={}", plusOne(1)) };
}
