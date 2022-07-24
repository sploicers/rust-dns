use super::{
    query_name_parser::{QueryName, QueryNameParser},
    query_type::QueryType,
    wrapped_buffer::WrappedBuffer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub query_type: QueryType,
}

impl DnsQuestion {
    fn new() -> DnsQuestion {
        DnsQuestion {
            name: String::new(),
            query_type: QueryType::UNKNOWN(0),
        }
    }

    pub fn read(buffer: &mut WrappedBuffer) -> Result<DnsQuestion, String> {
        let mut result = DnsQuestion::new();
        QueryName::read(buffer, &mut result.name)?;
        result.query_type = QueryType::from_u16(buffer.read_u16()?);
        buffer.read_u16()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        dns_question::DnsQuestion,
        test_helpers::{get_buffer_at_beginning, get_buffer_at_question_section},
        QueryType,
    };
    use std::error::Error;

    #[test]
    fn reads_name_successfully() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = String::from("google.com");
        let question = DnsQuestion::read(&mut get_buffer_at_question_section(String::from(
            "google_query.txt",
        ))?)?;
        assert_eq!(question.name.is_ascii(), true);
        assert_eq!(question.name, expected_domain_name);
        Ok(())
    }

    #[test]
    fn reads_type_successfully() -> Result<(), Box<dyn Error>> {
        let expected_type = QueryType::A;
        let question = DnsQuestion::read(&mut get_buffer_at_question_section(String::from(
            "google_query.txt",
        ))?)?;
        assert_eq!(question.query_type, expected_type);
        Ok(())
    }

    #[test]
    fn fails_to_read_name_if_buffer_at_wrong_pos() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = String::from("google.com");
        let question = DnsQuestion::read(&mut get_buffer_at_beginning(String::from(
            "google_query.txt",
        ))?)?;
        assert_ne!(question.name, expected_domain_name);
        assert_eq!(question.name.is_ascii(), false);
        Ok(())
    }

    #[test]
    #[ignore = "Need to make a hex dump of a query with unknown type"]
    fn reads_unknown_query_type_successfully() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
