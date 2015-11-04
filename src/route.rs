use std;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;
use std::net::IpAddr;
use std::net::Ipv4Addr;

use errors::RouteError;


#[derive(PartialEq, Debug)]
pub struct Route {
    pub iface: String,
    pub destination: std::net::IpAddr,
    pub gateway: std::net::IpAddr,
}

pub fn load() -> Result<Vec<Route>, RouteError> {
    let f = try!(File::open("/proc/net/route"));
    let reader = BufReader::new(f);

    parse_proc_net_route(reader)
}

fn parse_proc_net_route<T: BufRead>(reader: T) -> Result<Vec<Route>, RouteError> {
    // The first line is a header, we don't need it
    reader.lines().skip(1).map(|wrapped_line| {
        let line = try!(wrapped_line);
        let cols : Vec<&str> = line.split_whitespace().collect();
        match cols.as_slice() {
            [i, d, g, ..] =>
                Ok(Route { 
                    iface: i.to_string(),
                    destination: IpAddr::V4(try!(parse_ip(d))),
                    gateway: IpAddr::V4(try!(parse_ip(g))),
                }),
                _ =>
                    Err(RouteError::BadInput)
        }
    }).collect()
}

fn parse_ip(s: &str) -> Result<Ipv4Addr, RouteError> {
    let mut vec = Vec::new();
    for ch in s.chars().collect::<Vec<_>>().chunks(2) {
        let octect_str = ch.iter().cloned().collect::<String>();
        vec.push(try!(u8::from_str_radix(octect_str.as_str(), 16)));
    }

    if vec.len() != 4 {
        Err(RouteError::BadInput)
    } else {
        Ok(Ipv4Addr::new(vec[3], vec[2], vec[1], vec[0]))
    }
}


#[cfg(test)]
mod test {
    use std::io::BufReader;
    use std::fs::File;
    use std::net::IpAddr;
    use std::net::Ipv4Addr;

    use errors::RouteError;

    #[test]
    fn test_foo() {
        let f = File::open("tests/fixtures/sample").unwrap();
        let reader = BufReader::new(f);

        let vec = super::parse_proc_net_route(reader).unwrap();
        assert_eq!(vec[0], super::Route {
            iface: "eno1".to_string(),
            destination: IpAddr::V4(Ipv4Addr::new(0,0,0,0)),
            gateway: IpAddr::V4(Ipv4Addr::new(192,168,1,1))
        });
        assert_eq!(vec[1], super::Route {
            iface: "eno1".to_string(),
            destination: IpAddr::V4(Ipv4Addr::new(192,168,1,0)),
            gateway: IpAddr::V4(Ipv4Addr::new(0,0,0,0))
        });
        assert_eq!(vec[2], super::Route {
            iface: "virbr0".to_string(),
            destination: IpAddr::V4(Ipv4Addr::new(192,168,122,0)),
            gateway: IpAddr::V4(Ipv4Addr::new(0,0,0,0))
        });
    }

    #[test]
    fn parse_ip_returns_error_for_bad_input_0_length() {
        match super::parse_ip("000000") {
            Err(RouteError::BadInput) => (),
            otherwise => panic!("Expected RouteError::BadInput, got #{:?}", otherwise)
        }
    }

    #[test]
    fn parse_ip_returns_error_for_bad_input_length_short() {
        match super::parse_ip("000000") {
            Err(RouteError::BadInput) => (),
            otherwise => panic!("Expected RouteError::BadInput, got #{:?}", otherwise)
        }
    }


    #[test]
    fn parse_ip_returns_error_for_bad_input_length_long() {
        match super::parse_ip("0000000000") {
            Err(RouteError::BadInput) => (),
            otherwise => panic!("Expected RouteError::BadInput, got #{:?}", otherwise)
        }
    }

    #[test]
    fn parse_ip_returns_parse_error_for_bad_input_length_bad_character() {
        match super::parse_ip("000foo00") {
            Err(RouteError::Parse(_)) => (),
            otherwise => panic!("Expected RouteError::BadInput, got #{:?}", otherwise)
        }
    }

    #[test]
    fn it_works() {
        for x in super::load().unwrap() {
            println!("{} {} {}\n", x.iface, x.destination, x.gateway);
        }

    }
}
