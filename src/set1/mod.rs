
mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;

#[allow(dead_code)]
fn byte_hex_to_raw(digit: u8) -> u8
{
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => (digit - b'a') + 10,
        _ => panic!("Invalid Hex character")
    }
}

// How do i take a nibble here........hmmmm I may have to make a new type
#[allow(dead_code)]
fn byte_raw_to_hex(digit: u8) -> [u8; 2] {
    // matching the higher bits and the lower bits (nibbles)
    let (ah, al) = (digit >> 4, digit & 0x0f);
    let h = match ah {
        0..=9 => b'0' + ah,
        10..=15 => b'a' + (ah - 10),
        _ => panic!("a single hex char cannot represent digit > 15, invalid nibble ({digit})")
    };
    let l = match al {
        0..=9 => b'0' + al,
        10..=15 => b'a' + (al - 10),
        _ => panic!("a single hex char cannot represent digit > 15, invalid nibble ({digit})")
    };

    [h, l]

}


#[allow(dead_code)]
pub fn decode_hex_bad(s: &str) -> impl Iterator<Item = u8> + '_ {
    s.bytes().map(byte_hex_to_raw)
}

pub fn encode_hex(s: Vec<u8>) -> String {
    let utf8_bytes: Vec<u8> = s.into_iter().flat_map(byte_raw_to_hex).collect();
    // should be safe to use this here, coz only valid hex chars are returned from byte_raw_to_hex
    unsafe {
        String::from_utf8_unchecked(utf8_bytes)
    }
}

pub fn decode_hex(s: &str) -> Vec<u8>  {
    let chunked = s.as_bytes().chunks_exact(2);
    if chunked.remainder().len() != 0 {
        panic!("Why is there an extra nibble here lmao, hexstring len % 2 != 0");
    }

    chunked.into_iter().map(|chunk| {
        if let [ah, al] = *chunk {
            let ah = byte_hex_to_raw(ah);
            let al = byte_hex_to_raw(al);
            // ih = i higher order
            // ih = i lower order
            ah << 4 | al
        } else {
            unreachable!("no nibbles ???");
        }

    }).collect()
}