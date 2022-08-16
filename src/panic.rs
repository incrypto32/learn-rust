// You can use RUST_BACKTRACE=1 cargo run to get backtrace

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("PANIKKKK");
    }
}

pub fn run() {
    // Code for panicking
    // panic!("crash an burn");
    a();
}
