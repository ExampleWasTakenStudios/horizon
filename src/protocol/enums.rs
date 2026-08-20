#[derive(Debug, Clone)]
pub enum DnsType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
}

#[derive(Debug, Clone)]
pub enum DnsQType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AXFR = 252,
    WILD = 255,
}

#[derive(Debug, Clone)]
pub enum DnsClass {
    IN = 1,
}

#[derive(Debug, Clone)]
pub enum DnsQClass {
    IN = 1,
    WILD = 255,
}
