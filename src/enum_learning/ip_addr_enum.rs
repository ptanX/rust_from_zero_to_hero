pub enum IpAddrWithStringValue {
    V4(String),
    V6(String),
}

pub enum IpAddrWithArbitraryValue {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn create_ip_addr_with_string(kind: &String, value: &String) -> IpAddrWithStringValue {
    return match kind.as_str() {
        "V4" => IpAddrWithStringValue::V4(value.clone()),
        "V6" => IpAddrWithStringValue::V6(value.clone()),
        _ => panic!("Invalid IP address kind")
    }
}

pub fn get_ip_addr_with_string_value(ip_addr: &IpAddrWithStringValue) -> String {
    return match ip_addr {
        IpAddrWithStringValue::V4(value) => value.clone(),
        IpAddrWithStringValue::V6(value) => value.clone()
    };
}

pub fn get_ip_addr_with_arbitrary_value(ip_addr: &IpAddrWithArbitraryValue) -> String {
    return match ip_addr {
        IpAddrWithArbitraryValue::V4(v1, v2, v3, v4) => format!("{}{}{}{}",
                                                                v1.to_string(), v2.to_string(), v3.to_string(), v4.to_string()),
        IpAddrWithArbitraryValue::V6(value) => value.clone()
    };
}