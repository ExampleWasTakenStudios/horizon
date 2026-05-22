use crate::{buffer::PacketBuffer, protocol::ResponseCode};

#[derive(Debug)]
pub struct DomainName {
    /// A `Vec` containing all labels that make up the domain name.
    ///
    /// A label is stored as a `Vec` holding the individual bits.
    /// This results in the nested type structure `Vec<Vec<u8>>`.
    pub labels: Vec<Vec<u8>>,
}

impl DomainName {
    fn parse_raw(
        buffer: &[u8],
        start_position: usize,
    ) -> Result<(Vec<Vec<u8>>, usize), ResponseCode> {
        // The local cursor for parsing `buffer`.
        let mut local_position = start_position;
        // Indicates that a jump is active
        let mut jumped = false;
        // Holds the value to which `local_position` should return to, once the currently active jump is complete
        let mut return_position: usize = 0;

        // Holds all parsed labels - this will be returned
        let mut labels: Vec<Vec<u8>> = Vec::new();

        loop {
            let current_byte = *buffer.get(local_position).ok_or(ResponseCode::FORMERR)?;

            // Check if current_byte is a pointer
            if (current_byte & 0xC0) == 0xC0 {
                // Decode the pointer
                let low_byte = *buffer
                    .get(local_position + 1)
                    .ok_or(ResponseCode::FORMERR)? as u16;
                let ptr = ((((current_byte as u16) ^ 0xC0) << 8) | low_byte) as usize;

                // Check that pointer location is less than current position as per the RFC1035
                // This also mathematically prevents pointer loops as a new pointer must always be less than the current position.
                if ptr >= local_position {
                    return Err(ResponseCode::FORMERR);
                }

                // Check if a jump is currently active
                if jumped {
                    local_position = ptr;
                    continue;
                }

                return_position = local_position + 2;
                local_position = ptr;
                jumped = true;
                continue;
            }

            // current_byte is not a pointer so we check if it is a zero terminator (0x00)
            if current_byte == 0x00 {
                if jumped {
                    // If we jumped to local_position, we need to return to wherever we set return_position before we jumped.
                    local_position = return_position;
                } else {
                    // Since we didn't jump, we advance the local_position by one to move past the current position which is the zero terminator
                    local_position += 1;
                }
                break;
            }

            // current_byte is neither a pointer nor a zero terminator - therefore it must be a length byte indicating the length of the following label
            let length = current_byte as usize;
            local_position += 1;

            let label = buffer
                .get(local_position..local_position + length)
                .ok_or(ResponseCode::FORMERR)?
                .to_vec();
            labels.push(label);

            local_position += length;
        }

        Ok((labels, local_position - start_position))
    }

    pub fn read_from(buffer: &mut PacketBuffer) -> Result<Self, ResponseCode> {
        let (labels, bytes_consumed) = Self::parse_raw(buffer.as_slice(), buffer.get_position())?;

        buffer.advance_by(bytes_consumed);

        Ok(DomainName { labels })
    }
}
