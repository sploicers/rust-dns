use super::{
    parser_result::Result, query_name_parser::QueryNameParser, query_type::QueryType,
    wrapped_buffer::WrappedBuffer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub query_type: QueryType,
}

impl DnsQuestion {
    pub(crate) fn new(name: String, query_type: QueryType) -> DnsQuestion {
        DnsQuestion { name, query_type }
    }

    pub(crate) fn read(&mut self, buffer: &mut WrappedBuffer) -> Result<()> {
        QueryNameParser::from_buffer(buffer, &mut self.name)?;
        self.query_type = QueryType::from_u16(buffer.read_u16()?);
        buffer.read_u16()?;
        Ok(())
    }
}
