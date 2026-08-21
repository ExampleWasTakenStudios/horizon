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
        (opcode, count) => return Err(DnsError::UnsupportedQuestionCount { opcode, count }),
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
mod domain_name_tests {
    use hex_literal::hex;

    use super::*;

    static VALID_QUERY: [u8; 93] = hex!(
        "
        74 42 7f 8a 90 b3 54 8d 5a 1f 9b 59 08 00 45 00
        00 4f 91 ec 00 00 40 11 25 06 c0 a8 01 02 01 01
        01 01 b5 e8 00 35 00 3b cd 67 9b 4c 01 20 00 01
        00 00 00 00 00 01 06 67 6f 6f 67 6c 65 03 63 6f
        6d 00 00 01 00 01 00 00 29 04 d0 00 00 00 00 00
        0c 00 0a 00 08 bf cd b6 13 88 d5 1c 3b
    "
    );

    static valid_response: [u8; 97] = hex!(
        "
        54 8d 5a 1f 9b 59 74 42 7f 8a 90 b3 08 00 45 00
        00 53 d4 1c 40 00 3b 11 a7 d1 01 01 01 01 c0 a8
        01 02 00 35 b5 e8 00 3f e7 e5 9b 4c 81 80 00 01
        00 01 00 00 00 01 06 67 6f 6f 67 6c 65 03 63 6f
        6d 00 00 01 00 01 c0 0c 00 01 00 01 00 00 00 b7
        00 04 8e fb 25 6e 00 00 29 04 d0 00 00 00 00 00
        00
    "
    );

    #[test]
    fn test_root_domain_only_packets_correctly_parsed() {
        todo!()
    }

    #[test]
    fn test_uncompressed_name_correctly_parsed() {
        todo!()
    }

    #[test]
    fn test_single_pointer_correctly_parsed() {
        todo!()
    }

    #[test]
    fn test_nested_pointers_correctly_parsed() {
        todo!()
    }

    #[test]
    fn test_infinite_pointer_loop_caught() {
        todo!()
    }

    #[test]
    fn test_self_referential_pointer_caught() {
        todo!()
    }

    #[test]
    fn test_forward_pointer_caught() {
        todo!()
    }

    #[test]
    fn test_jump_limit_respected() {
        todo!()
    }

    #[test]
    fn test_too_long_label_caught() {
        todo!()
    }

    #[test]
    fn test_too_long_nam_caught() {
        todo!()
    }

    #[test]
    fn test_missing_zero_terminator_caught() {}
}

#[cfg(test)]
mod header_tests {
    use super::*;

    #[test]
    fn test_all_zeros_correctly_parsed() {
        todo!()
    }

    #[test]
    fn test_flag_parsing() {
        todo!()
    }

    #[test]
    fn test_counts() {
        todo!()
    }

    #[test]
    fn test_buffer_too_short_caught() {
        todo!()
    }

    #[test]
    fn test_cursor_not_at_zero() {
        todo!()
    }
}

#[cfg(test)]
mod question_tests {
    use super::*;

    #[test]
    fn test_question_count_is_correct() {
        todo!()
    }

    #[test]
    fn test_empty_query_has_no_question() {
        todo!()
    }

    #[test]
    fn test_rfc_9619_only_one_question() {
        todo!()
    }

    #[test]
    fn test_queries_with_multiple_questions_refused() {
        todo!()
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;

    #[test]
    fn test_record_parsing() {
        todo!()
    }

    #[test]
    fn test_multiple_records_parsing() {
        todo!()
    }

    #[test]
    fn test_rdlength_mismatch_caught() {
        todo!()
    }

    #[test]
    fn test_zero_rdlength_is_accepted() {
        todo!()
    }
}
