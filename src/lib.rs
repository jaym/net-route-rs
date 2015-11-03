#![feature(ip_addr)]
#![feature(slice_patterns)]
#![feature(convert)]

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::num;

#[derive(Debug)]
pub enum RouteError {
    Io(io::Error),
    Parse(num::ParseIntError),
    BadInput
}

impl From<io::Error> for RouteError {
    fn from(err: io::Error) -> RouteError {
        RouteError::Io(err)
    }
}

impl From<num::ParseIntError> for RouteError {
    fn from(err: num::ParseIntError) -> RouteError {
        RouteError::Parse(err)
    }
}

pub struct Route {
    pub iface: String,
    pub destination: std::net::IpAddr,
    pub gateway: std::net::IpAddr,
}

impl Route {
    pub fn load() -> Result<Vec<Route>, RouteError> {
        let f = try!(File::open("/proc/net/route"));
        let reader = BufReader::new(f);

        // The first line is a header, we don't need it
        reader.lines().skip(1).map(|wrapped_line| {
            let line = try!(wrapped_line);
            let cols : Vec<&str> = line.split_whitespace().collect();
            match cols.as_slice() {
                [i, d, g, ..] =>
                    Ok(Route { 
                        iface: i.to_string(),
                        destination: IpAddr::V4(try!(Route::parse_ip(d))),
                        gateway: IpAddr::V4(try!(Route::parse_ip(g))),
                    }),
                _ =>
                    Err(RouteError::BadInput)
            }
        }).collect()
    }

    pub fn parse_ip(s: &str) -> Result<Ipv4Addr, RouteError> {
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
}

#[test]
fn it_works() {
    for x in Route::load().unwrap() {
        println!("{} {} {}\n", x.iface, x.destination, x.gateway);
    }

}
