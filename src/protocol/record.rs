#[derive(Debug, Clone)]
pub struct DnsRecord<'a> {
    name: &'a [u8],
    r#type: &'a [u8],
    class: &'a [u8],
    ttl: &'a [u8],
    rd_length: &'a [u8],
    r_data: &'a [u8],
}
