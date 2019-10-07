pub mod linkerd {
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.net.rs"));
    }

    pub mod http_types {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.http_types.rs"));
    }

    pub mod destination {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.destination.rs"));
    }

    // ===== impl net::IpAddress =====

    impl<T> From<T> for net::IpAddress
    where
        net::ip_address::Ip: From<T>,
    {
        #[inline]
        fn from(ip: T) -> Self {
            Self {
                ip: Some(ip.into()),
            }
        }
    }

    impl From<::std::net::IpAddr> for net::IpAddress {
        fn from(ip: ::std::net::IpAddr) -> Self {
            match ip {
                ::std::net::IpAddr::V4(v4) => Self {
                    ip: Some(v4.into()),
                },
                ::std::net::IpAddr::V6(v6) => Self {
                    ip: Some(v6.into()),
                },
            }
        }
    }

    impl From<[u8; 4]> for net::ip_address::Ip {
        fn from(octets: [u8; 4]) -> Self {
            net::ip_address::Ip::Ipv4(
                u32::from(octets[0]) << 24 | u32::from(octets[1]) << 16 | u32::from(octets[2]) << 8
                    | u32::from(octets[3]),
            )
        }
    }

    // ===== impl net::ip_address:Ip =====

    impl From<::std::net::Ipv4Addr> for net::ip_address::Ip {
        #[inline]
        fn from(v4: ::std::net::Ipv4Addr) -> Self {
            Self::from(v4.octets())
        }
    }

    impl<T> From<T> for net::ip_address::Ip
    where
        net::IPv6: From<T>,
    {
        #[inline]
        fn from(t: T) -> Self {
            net::ip_address::Ip::Ipv6(net::IPv6::from(t))
        }
    }

    // ===== impl net::IPv6 =====

    impl From<[u8; 16]> for net::IPv6 {
        fn from(octets: [u8; 16]) -> Self {
            let first = (u64::from(octets[0]) << 56) + (u64::from(octets[1]) << 48)
                + (u64::from(octets[2]) << 40) + (u64::from(octets[3]) << 32)
                + (u64::from(octets[4]) << 24) + (u64::from(octets[5]) << 16)
                + (u64::from(octets[6]) << 8) + u64::from(octets[7]);
            let last = (u64::from(octets[8]) << 56) + (u64::from(octets[9]) << 48)
                + (u64::from(octets[10]) << 40) + (u64::from(octets[11]) << 32)
                + (u64::from(octets[12]) << 24) + (u64::from(octets[13]) << 16)
                + (u64::from(octets[14]) << 8) + u64::from(octets[15]);
            Self {
                first,
                last,
            }
        }
    }

    impl From<::std::net::Ipv6Addr> for net::IPv6 {
        #[inline]
        fn from(v6: ::std::net::Ipv6Addr) -> Self {
            Self::from(v6.octets())
        }
    }

    impl<'a> From<&'a net::IPv6> for ::std::net::Ipv6Addr {
        fn from(ip: &'a net::IPv6) -> ::std::net::Ipv6Addr {
            ::std::net::Ipv6Addr::new(
                (ip.first >> 48) as u16,
                (ip.first >> 32) as u16,
                (ip.first >> 16) as u16,
                (ip.first) as u16,
                (ip.last >> 48) as u16,
                (ip.last >> 32) as u16,
                (ip.last >> 16) as u16,
                (ip.last) as u16,
            )
        }
    }

    // ===== impl net::TcpAddress =====

    impl<'a> From<&'a ::std::net::SocketAddr> for net::TcpAddress {
        fn from(sa: &::std::net::SocketAddr) -> net::TcpAddress {
            net::TcpAddress {
                ip: Some(sa.ip().into()),
                port: u32::from(sa.port()),
            }
        }
    }
}
