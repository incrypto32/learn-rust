// Enums can be different types.

// This is an example of an enum with two variants.
#[derive(Debug)]
enum YesOrNo {
    Yes,
    No,
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// The earlier example doesn not have actual data in the enum.
// This is an example of an enum with actual data.
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        println!(
            "{:?}",
            match self {
                IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
                IpAddr::V6(a) => a.to_string(),
            }
        );
    }
}

pub fn run() {
    // Here we match on the value of the enum.
    // It is kind of used as a type
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    println!("v4 is a v4 ip address: {:?}", v4);
    println!("v6 is a v6 ip address: {:?}", v6);

    // Similar code for YesOrNo struct
    let yes = YesOrNo::Yes;
    let no = YesOrNo::No;

    println!("yes is {:?}", yes);
    println!("no is {:?}", no);

    // Similar code for ipAddr struct
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
    home.print();
    loopback.print();
}
