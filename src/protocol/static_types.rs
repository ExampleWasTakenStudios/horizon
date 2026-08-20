#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsType(pub u16);
impl DnsType {
    pub const A: Self = Self(1);
    pub const NS: Self = Self(2);
    pub const CNAME: Self = Self(5);
    pub const SOA: Self = Self(6);
    pub const PTR: Self = Self(12);
    pub const MX: Self = Self(15);
    pub const TXT: Self = Self(16);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQType(pub u16);
impl DnsQType {
    pub const A: Self = Self(1);
    pub const NS: Self = Self(2);
    pub const CNAME: Self = Self(5);
    pub const SOA: Self = Self(6);
    pub const PTR: Self = Self(12);
    pub const MX: Self = Self(15);
    pub const TXT: Self = Self(16);
    pub const AXFR: Self = Self(252);
    pub const WILD: Self = Self(255);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsClass(pub u16);
impl DnsClass {
    pub const IN: Self = Self(1);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQClass(pub u16);
impl DnsQClass {
    pub const IN: Self = Self(1);
    pub const WILD: Self = Self(255);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsOpCode(pub u8);
impl DnsOpCode {
    pub const QUERY: Self = Self(0);
}
