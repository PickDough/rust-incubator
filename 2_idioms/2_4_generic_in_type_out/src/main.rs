use std::net::{AddrParseError, IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER".to_string());
    err.status(404).message("User not found".to_string());
}

#[derive(Debug)]
pub struct Error {
    code: String,
    status: u16,
    message: String,
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".to_string(),
            status: 500,
            message: "Unknown error has happened.".to_string(),
        }
    }
}

impl Error {
    pub fn new<S: Into<String>>(code: S) -> Self {
        Self { code: code.into(), ..Default::default() }
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message<S: Into<String>>(&mut self, m: S) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

pub enum ServerTuple<'a> {
    Str(&'a str),
    Ip(IpAddr),
}

impl<'a> From<&'a str> for ServerTuple<'a> {
    fn from(value: &'a str) -> ServerTuple<'a> {
        ServerTuple::Str(value)
    }
}

impl<'a> From<IpAddr> for ServerTuple<'a> {
    fn from(value: IpAddr) -> Self {
        ServerTuple::Ip(value)
    }
}

impl<'a> TryInto<IpAddr> for ServerTuple<'a> {
    type Error = AddrParseError;

    fn try_into(self) -> Result<IpAddr, Self::Error> {
        match self {
            ServerTuple::Str(s) => {
                s.parse()
            }
            ServerTuple::Ip(ip) => {
                Ok(ip)
            }
        }
    }
}


impl Server {
    pub fn bind<'a, S: Into<ServerTuple<'a>>>(&mut self, ip: S, port: u16) {
        self.0 = ip.into().try_into().map_or_else(|_| None, |i| Some(SocketAddr::new(i, port)));
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind("::1", 9911);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
