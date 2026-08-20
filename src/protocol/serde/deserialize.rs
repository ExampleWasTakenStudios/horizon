use arrayvec::ArrayVec;

use crate::{
    error::{DnsError, DnsResult},
    protocol::{
        DnsCursor, DnsHeader, DnsMessage, DnsQuestion, DnsRecord, DomainName, MAX_NAME_LENGTH,
        RawMessage,
    },
};

pub fn deserialize(buf: RawMessage) -> DnsResult<DnsMessage> {
    let mut cursor = DnsCursor::new(buf);

    let header = deserialize_header(&mut cursor)?;

    // RFC 9619 bans queries with more than one question. Consequently, we check the QDCOUNT field of the header before continuing.
    if header.op_code == 0 && header.question_count > 1 {
        todo!(
            "From RFC 9619: A DNS message with OPCODE = 0 and QDCOUNT > 1 MUST be treated as an incorrectly formatted message. The value of the RCODE parameter in the response message MUST be set to 1 (FORMERR)."
        );
    }

    let questions = deserialize_question(&mut cursor, header.question_count)?;
    let answers = deserialize_records(&mut cursor, header.answer_count)?;
    let authoritatives = deserialize_records(&mut cursor, header.authoritative_count)?;
    let additionals = deserialize_records(&mut cursor, header.additional_count)?;

    Ok(DnsMessage::new(
        header,
        questions,
        answers,
        authoritatives,
        additionals,
    ))
}

fn deserialize_header(cursor: &mut DnsCursor) -> DnsResult<DnsHeader> {
    if cursor.pos() != 0 {
        return Err(DnsError::CursorPositionNotAtZeroWhileDeserializingHeader);
    }
    if cursor.len() <= 12 {
        return Err(DnsError::PacketTooShort(Box::new(cursor.clone_buf())));
    }

    let id = cursor.read_u16()?;

    let flags = cursor.read_u16()?;
    let is_response: bool = (flags >> 15) != 0;
    let op_code: u8 = ((flags >> 11) & 0xF) as u8;
    let is_authoritative: bool = ((flags >> 10) & 0x1) != 0;
    let is_truncated: bool = ((flags >> 9) & 0x1) != 0;
    let recursion_desired: bool = (flags >> 8 & 0x1) != 0;
    let recursion_avail: bool = ((flags >> 7) & 0x1) != 0;
    let z: u8 = ((flags >> 4) & 0x5) as u8;
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

fn deserialize_question(cursor: &DnsCursor, amount: u16) -> DnsResult<DnsQuestion> {
    todo!()
}

fn deserialize_records(cursor: &DnsCursor, amount: u16) -> DnsResult<Vec<DnsRecord>> {
    todo!()
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
