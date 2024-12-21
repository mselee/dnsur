#![forbid(unsafe_code)]
// #![forbid(missing_docs, future_incompatible)]

pub mod errors;
mod iter;
mod lookup;
#[cfg(unix)]
mod parser;
mod resolvers;
use std::{
    collections::BTreeSet,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

// #[cfg(unix)]

#[derive(Debug, Clone, PartialEq)]
pub struct HostEntry {
    pub ip: IpAddr,
    pub hosts: BTreeSet<String>,
}

impl HostEntry {
    pub fn new(ip: IpAddr, hosts: impl Iterator<Item = String>) -> Self {
        Self {
            ip,
            hosts: hosts.collect(),
        }
    }
}

pub struct DnsResolver {
    entries: Vec<HostEntry>,
    search: Vec<String>,
    nameservers: Vec<SocketAddr>,
    timeout: Duration,
    ndots: u8,
    attempts: u8,
    rotate: bool,
    udp_payload_size: u16,
}
