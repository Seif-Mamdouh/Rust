// Enums
// Enums is way of saying that a value is one possible set of values.
// custom data type that lets you define a type by enumerating its possible values.


enum IpAddrKind {
    V4,
    V6,
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("Ip address 4"),
    };

    println!("The address is: {}", home.address);
}


// we can also use enums to define different types of data
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }