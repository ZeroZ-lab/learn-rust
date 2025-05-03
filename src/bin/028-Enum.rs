enum IPAddress {
    V4,
    V6,
}

fn main() {
    let ip1 = IPAddress::V4;
    let ip2 = IPAddress::V6;

    route(ip1);
    route(ip2);
}

fn route(ip_type: IPAddress) {
    match ip_type {
        IPAddress::V4 => println!("v4"),
        IPAddress::V6 => println!("v6"),
    }
}
