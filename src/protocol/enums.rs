#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResponseCode {
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
    pub fn from_number(number: u8) -> ResponseCode {
        match number {
            1 => ResponseCode::FORMERR,
            2 => ResponseCode::SERVFAIL,
            3 => ResponseCode::NXDOMAIN,
            4 => ResponseCode::NOTIMP,
            5 => ResponseCode::REFUSED,
            0 | _ => ResponseCode::NOERROR,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DnsClass {
    IN = 1,
    WILDCARD = 255,
}

impl DnsClass {
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
    pub fn from_number(x: u16) -> Self {
        match x {
            1 => DnsClass::IN,
            255 | _ => DnsClass::WILDCARD,
        }
    }

    pub fn to_number(&self) -> u16 {
        match *self {
            DnsClass::IN => 1,
            DnsClass::WILDCARD => 255,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DnsType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    WILDCARD = 255,
}

impl DnsType {
    #[allow(clippy::wildcard_in_or_patterns)] // Added for readability
    pub fn from_number(x: u16) -> Self {
        match x {
            1 => DnsType::A,
            2 => DnsType::NS,
            5 => DnsType::CNAME,
            6 => DnsType::SOA,
            12 => DnsType::PTR,
            15 => DnsType::MX,
            16 => DnsType::TXT,
            255 | _ => DnsType::WILDCARD,
        }
    }

    pub fn to_number(&self) -> u16 {
        match *self {
            DnsType::A => 1,
            DnsType::NS => 2,
            DnsType::CNAME => 5,
            DnsType::SOA => 6,
            DnsType::PTR => 12,
            DnsType::MX => 15,
            DnsType::TXT => 16,
            DnsType::WILDCARD => 255,
        }
    }
}
