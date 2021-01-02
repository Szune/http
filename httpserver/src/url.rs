use crate::buffer::Buffered;

pub struct QueryPart {
    pub key: String,
    pub value: String,
}

pub struct Url {
    pub fragments: Vec<String>,
    pub query: Vec<QueryPart>,
}

pub fn parse(s: &str) -> Url {
    todo!();
    Url {
        fragments: Vec::new(),
        query: Vec::new(),
    }
}


/*
match b {
    0x25 => {
        match raw_iter.peek(0).unwrap_or(&0) {
            unit1 @ 0x30..=0x39 | unit1 @ 0x41..=0x46 | unit1 @ 0x61..=0x66 => {
                match raw_iter.peek(1).unwrap_or(&0) {
                    unit2 @ 0x30..=0x39 | unit2 @ 0x41..=0x46 | unit2 @ 0x61..=0x66 => {
                        let dbyte = u8::from_str_radix(
                            String::from_utf8(vec![*unit1, *unit2]).unwrap().as_str(),
                            16).unwrap();
                        decoded.push(dbyte);
                        skip += 2;
                    },
                    _ => decoded.push(*b),
                }
            },
            _ => decoded.push(*b),
        }
    },
    _ => decoded.push(*b),
}

 */