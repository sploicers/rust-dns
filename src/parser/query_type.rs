#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
#[allow(dead_code)]
pub enum QueryType {
    UNKNOWN(u16),
    A,
}

impl QueryType {
    pub fn from_u16(val: u16) -> QueryType {
        match val {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(val),
        }
    }
}
