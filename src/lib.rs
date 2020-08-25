use std::default::Default;
use std::net::{ TcpListener, Ipv4Addr };

#[derive(Debug)]
pub struct PortRange {
    pub min: u16,
    pub max: u16
}

impl Default for PortRange {
    fn default() -> Self {
        PortRange { min: 1024, max: 65535 }
    }
}

pub fn get_port_in_range(range: PortRange) -> Option<u16> {
    return (range.min..=(range.max)).filter(|p| check_port_available(&p)).nth(0); 
}


pub fn get_port() -> Option<u16> {
    return get_port_in_range(PortRange::default());
}

pub fn get_port_prefer(ports: Vec<u16>) -> Option<u16> {
    let port = ports.into_iter().filter(|p| check_port_available(&p)).nth(0);

    if port.is_some() {
       return port;
    } else {
       return get_port();
    }
}

pub fn get_port_except(ports: Vec<u16>) -> Option<u16> {
    return (PortRange::default().min..=PortRange::default().max).filter(|p| !ports.contains(p) && check_port_available(p)).nth(0);
}

pub fn get_port_in_range_except(range: PortRange, ports: Vec<u16>) -> Option<u16> {
    return (range.min..=range.max).filter(|p| !ports.contains(p) && check_port_available(p)).nth(0);
}

fn check_port_available(port: &u16) -> bool {
    match TcpListener::bind((Ipv4Addr::LOCALHOST, *port)) {
        Ok(_) => return true,
        Err(_) => return false
    };
}