#[derive(Debug, Clone)]
pub struct DnsQuestion<'a> {
    q_name: &'a [u8],
    q_type: &'a [u8],
    q_class: &'a [u8],
}
