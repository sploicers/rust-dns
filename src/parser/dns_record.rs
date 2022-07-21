use super::parser_result::Result;
use std::net::Ipv4Addr;

use super::{
    query_name_parser::QueryNameParser, query_type::QueryType, wrapped_buffer::WrappedBuffer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        query_type: u16,
        data_length: u16,
        ttl: u32,
    },
    A {
        domain: String,
        address: Ipv4Addr,
        ttl: u32,
    },
}

impl DnsRecord {
    pub(crate) fn read(buffer: &WrappedBuffer) -> Result<DnsRecord> {
        let domain = String::new();
        QueryNameParser::from_buffer(buffer, &mut domain)?;

        let query_type_num = buffer.read_u16()?;
        let query_type = QueryType::from_u16(query_type_num);
        buffer.read_u16()?;

        let ttl = buffer.read_u32()?;
        let data_length = buffer.read_u16()?;

        match query_type {
            QueryType::A => {
                let raw_address = buffer.read_u32()?;
                let address = Ipv4Addr::new(
                    ((raw_address >> 24) & 0xFF) as u8,
                    ((raw_address >> 24) & 0xFF) as u8,
                    ((raw_address >> 24) & 0xFF) as u8,
                    ((raw_address >> 24) & 0xFF) as u8,
                );
                Ok(DnsRecord::A {
                    domain,
                    address,
                    ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
                buffer.advance(data_length as usize)?;
                let query_type = query_type_num;
                Ok(DnsRecord::UNKNOWN {
                    domain,
                    query_type,
                    data_length,
                    ttl,
                })
            }
        }
    }
}
