enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {

    let ip1 = IPAddress::V4(127, 0, 0, 1);
    let ip2 = IPAddress::V6(String::from("::1"));

    route(ip1);
    route(ip2);
}

fn route(ip_type: IPAddress) {
    match ip_type {
        IPAddress::V4(a, b, c, d) => println!("v4: {}.{}.{}.{}", a, b, c, d),
        IPAddress::V6(s) => println!("v6: {}", s),
    }
}