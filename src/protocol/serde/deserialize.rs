use crate::{
    error::{DnsError, DnsResult},
    protocol::{
        DnsClass, DnsCursor, DnsHeader, DnsMessage, DnsQClass, DnsQType, DnsQuestion, DnsRecord,
        DnsType, DomainName, MAX_NAME_LENGTH, RawMessage,
    },
};
use arrayvec::ArrayVec;

pub fn deserialize(buf: RawMessage) -> DnsResult<DnsMessage> {
    let mut cursor = DnsCursor::new(buf);

    let header = deserialize_header(&mut cursor)?;

    // RFC 9619 bans queries with more than one question. Consequently, we check the QDCOUNT field of the header before continuing.

    let question = match (header.op_code, header.question_count) {
        // 1. Strict RFC 9619 Compliance
        // If it's a standard query with >1 question, it is explicitly malformed.
        (0, count) if count > 1 => return Err(DnsError::TooManyQuestions),

        // 2. Standard Happy Path
        // Exactly 1 question. We deserialize it and wrap it in Some.
        (_, 1) => Some(deserialize_question(&mut cursor)?),

        // 3. Zero Path
        // Valid for server responses across various OpCodes.
        (_, 0) => None,

        // 4. Structural Limit
        // If it's an obscure OpCode that somehow has >1 questions,
        // out `DnsMessage` struct fundamentally cannot represent it via Option<DnsQuestion>.
        // We must reject it here to protect the data model.
        (opcode, count) => return Err(DnsError::UnsupportedQuestionCount { opcode, count })
    };

    let answers = deserialize_records(&mut cursor, header.answer_count)?;
    let authoritatives = deserialize_records(&mut cursor, header.authoritative_count)?;
    let additionals = deserialize_records(&mut cursor, header.additional_count)?;

    Ok(DnsMessage::new(
        header,
        question,
        answers,
        authoritatives,
        additionals,
    ))
}

fn deserialize_header(cursor: &mut DnsCursor) -> DnsResult<DnsHeader> {
    if cursor.pos() != 0 {
        return Err(DnsError::CursorPositionNotAtZeroWhileDeserializingHeader);
    }
    if cursor.len() < 12 {
        return Err(DnsError::PacketTooShort(Box::new(cursor.clone_buf())));
    }

    let id = cursor.read_u16()?;

    let flags = cursor.read_u16()?;

    let is_response: bool = (flags >> 15) != 0;
    let op_code: u8 = ((flags >> 11) & 0xF) as u8;
    let is_authoritative: bool = ((flags >> 10) & 0x1) != 0;
    let is_truncated: bool = ((flags >> 9) & 0x1) != 0;
    let recursion_desired: bool = ((flags >> 8) & 0x1) != 0;
    let recursion_avail: bool = ((flags >> 7) & 0x1) != 0;
    let z: u8 = ((flags >> 4) & 0x7) as u8;
    let response_code: u8 = (flags & 0xF) as u8;

    let question_count = cursor.read_u16()?;
    let answer_count = cursor.read_u16()?;
    let authoritative_count = cursor.read_u16()?;
    let additional_count = cursor.read_u16()?;

    Ok(DnsHeader {
        id,
        is_response,
        op_code,
        is_authoritative,
        is_truncated,
        recursion_desired,
        recursion_avail,
        z,
        response_code,
        question_count,
        answer_count,
        authoritative_count,
        additional_count,
    })
}

fn deserialize_question(cursor: &mut DnsCursor) -> DnsResult<DnsQuestion> {
    let q_name = deserialize_domain_name(cursor)?;
    let q_type = DnsQType(cursor.read_u16()?);
    let q_class = DnsQClass(cursor.read_u16()?);

    Ok(DnsQuestion {
        q_name,
        q_type,
        q_class,
    })
}

fn deserialize_records(cursor: &mut DnsCursor, amount: u16) -> DnsResult<Vec<DnsRecord>> {
    let mut records = Vec::<DnsRecord>::with_capacity(amount as usize);

    for _ in 0..amount {
        let name = deserialize_domain_name(cursor)?;
        let r#type = DnsType(cursor.read_u16()?);
        let class = DnsClass(cursor.read_u16()?);
        let ttl = cursor.read_u32()?;
        let rd_length = cursor.read_u16()?;
        let r_data = cursor.read_slice(rd_length as usize)?.to_vec();

        records.push(DnsRecord {
            name,
            r#type,
            class,
            ttl,
            rd_length,
            r_data,
        });
    }

    Ok(records)
}

fn deserialize_domain_name(cursor: &mut DnsCursor) -> DnsResult<DomainName> {
    // Indicates that the current cursor position is in a jumped state.
    let mut jumped = false;

    // Used to cap the maximum number of jumps
    let mut jump_counter = 0_u8;
    const MAX_JUMP_COUNT: u8 = 10;

    // The position of the cursor prior to the first jump.
    let mut ptr_pos = 0_usize;

    let mut domain_name = ArrayVec::<u8, MAX_NAME_LENGTH>::new();

    const PTR_ID_MASK: u8 = 0xC0;
    const PTR_HIGH_BIT_MASK: u8 = 0x3F;

    loop {
        let length = cursor.read_u8()?;

        // Is `length` a pointer
        if length & PTR_ID_MASK == 0xC0 {
            if jump_counter >= MAX_JUMP_COUNT {
                return Err(DnsError::TooManyPointersInDomainName);
            }

            let ptr_high_byte: u16 = ((length & PTR_HIGH_BIT_MASK) as u16) << 8;
            let ptr_low_byte: u16 = cursor.read_u8()? as u16;

            let ptr: u16 = ptr_high_byte | ptr_low_byte;

            // Ensure the pointer is backwards and does not exceed the current cursor position - i.e. pointing to memory in front of the cursor
            // The -2 is because by the time we check the cursor position here, the cursor has moved on two bytes caused by the reading of the high and low byte of the pointer.
            if ptr as usize >= cursor.pos() - 2 {
                return Err(DnsError::IllegalPointer);
            }

            // We only store the original position of the cursor when we are not already jumped since we don't want to unwrap the nested jumps.
            if !jumped {
                ptr_pos = cursor.pos();
            }

            cursor.set_pos(ptr as usize);
            jumped = true;
            jump_counter += 1;
            continue;
        }

        // Is `length` a zero terminator which would indicate the end of the domain-name
        if length == 0x00 {
            if domain_name.try_push(length).is_err() {
                return Err(DnsError::DomainNameTooLong);
            }

            break;
        }

        // Since `length` is neither a pointer nor a zero terminator, its a normal length indicator so we can read that many bytes at once

        // Check that the label does not exceed 63 bytes as specified in RFC 1035
        if length > 63 {
            return Err(DnsError::DomainNameLabelTooLong);
        }

        if domain_name.try_push(length).is_err() {
            return Err(DnsError::DomainNameTooLong);
        };
        if domain_name
            .try_extend_from_slice(cursor.read_slice(length as usize)?)
            .is_err()
        {
            return Err(DnsError::DomainNameTooLong);
        };
    }

    // If we jumped and thus modified the cursor position, we reset it to the value of the pointer
    if jumped {
        cursor.set_pos(ptr_pos);
    }

    Ok(DomainName::new(domain_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_valid_domain_name() {}
}
