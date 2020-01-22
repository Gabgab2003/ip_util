use std::env::args;
use ip_util::IpAddress;

fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() <= 1 {
        eprintln!("Usage: ipinfo [ips]");
        return;
    }
    for i in 1..arguments.len() {
        print_info(&arguments[i]);
    }
}

fn print_info(a: &str) {
    let (ip, subnetmask) = split_info(a);
    let subnet_repr = subnetmask_repr(&subnetmask);
    let network_addr = ip & subnetmask;
    println!("{}|{}|{}", subnet_repr, subnetmask, network_addr);
}



fn split_info(st: &str) -> (IpAddress, IpAddress) {
    let parts: Vec<&str> = st.split("/").collect();
    let addr = IpAddress::parse_ip(parts.get(0).unwrap()).unwrap();
    let subnet_length = parts.get(1).unwrap().parse::<u8>().unwrap();
    let subnet_mask = IpAddress::subnetmask(subnet_length).unwrap();
    (addr, subnet_mask)
}

fn subnetmask_repr(ip: &IpAddress) -> String {
    let mut st = String::new();
    for i in 0..4 {
        let byte = ip.bytes[i];
        if byte == 0 {
            st.push('H');
        } else if byte == 255 {
            st.push('N');
        } else {
            for i in 1..=8 {
                let chr = match byte.rotate_left(i % 8) & 1 {
                    0 => 'h',
                    1 => 'n',
                    _ => unreachable!("x & 1 should be 0 or 1")
                };
                st.push(chr);
            };
        }
        if i != 3 {
            st.push('.')
        }
    };
    return st;
}