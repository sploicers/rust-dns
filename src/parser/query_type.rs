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

    pub fn to_u16(&self) -> u16 {
        match *self {
            QueryType::A => 1,
            QueryType::UNKNOWN(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryType;

    #[test]
    fn gets_value_for_known_type() {
        assert_eq!(QueryType::A, QueryType::from_u16(1));
    }

    #[test]
    fn gets_unknown_value_for_unknown_type() {
        assert_eq!(QueryType::UNKNOWN(0), QueryType::from_u16(0));
        assert_eq!(QueryType::UNKNOWN(999), QueryType::from_u16(999));
    }
}
