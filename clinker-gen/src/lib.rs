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
}
