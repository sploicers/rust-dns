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

    pub fn write(&self, buffer: &mut WrappedBuffer) -> Result<(), String> {
        QueryName::write(buffer, &self.name)?;
        buffer.write_u16(self.query_type.to_u16())?;
        buffer.write_u16(1)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        dns_question::DnsQuestion,
        test_helpers::{get_buffer_at_beginning, get_buffer_at_question_section, GOOGLE_QUERY},
        wrapped_buffer::WrappedBuffer,
        QueryType,
    };
    use std::error::Error;

    #[test]
    fn reads_name_successfully() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = String::from("google.com");
        let question = DnsQuestion::read(&mut get_buffer_at_question_section(String::from(
            GOOGLE_QUERY,
        ))?)?;
        assert_eq!(question.name.is_ascii(), true);
        assert_eq!(question.name, expected_domain_name);
        Ok(())
    }

    #[test]
    fn reads_type_successfully() -> Result<(), Box<dyn Error>> {
        let expected_type = QueryType::A;
        let question = DnsQuestion::read(&mut get_buffer_at_question_section(String::from(
            GOOGLE_QUERY,
        ))?)?;
        assert_eq!(question.query_type, expected_type);
        Ok(())
    }

    #[test]
    fn fails_to_read_name_if_buffer_at_wrong_pos() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = String::from("google.com");
        let question =
            DnsQuestion::read(&mut get_buffer_at_beginning(String::from(GOOGLE_QUERY))?)?;
        assert_ne!(question.name, expected_domain_name);
        assert_eq!(question.name.is_ascii(), false);
        Ok(())
    }

    #[test]
    #[ignore = "Need to make a hex dump of a query with unknown type"]
    fn reads_unknown_query_type_successfully() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    #[test]
    fn writes_name_successfully() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = "google.com";

        let mut buffer = WrappedBuffer::new();
        let question = DnsQuestion {
            name: String::from(expected_domain_name),
            query_type: QueryType::A,
        };

        question.write(&mut buffer)?;
        buffer.seek(0)?;
        let result = DnsQuestion::read(&mut buffer)?;

        assert_eq!(result.name, expected_domain_name);
        Ok(())
    }

    #[test]
    fn writes_type_successfully() -> Result<(), Box<dyn Error>> {
        let expected_query_type = QueryType::A;

        let mut buffer = WrappedBuffer::new();
        let question = DnsQuestion {
            name: String::from("github.ru"),
            query_type: expected_query_type,
        };

        question.write(&mut buffer)?;
        buffer.seek(0)?;
        let result = DnsQuestion::read(&mut buffer)?;

        assert_eq!(result.query_type, expected_query_type);
        Ok(())
    }
}
