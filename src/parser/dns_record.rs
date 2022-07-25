use std::net::Ipv4Addr;

use super::{
    bitshifting::get_nth_octal,
    query_name_parser::{QueryName, QueryNameParser},
    query_type::QueryType,
    wrapped_buffer::WrappedBuffer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn read(buffer: &mut WrappedBuffer) -> Result<DnsRecord, String> {
        let mut domain = String::new();
        QueryName::read(buffer, &mut domain)?;

        let query_type_num = buffer.read_u16()?;
        let query_type = QueryType::from_u16(query_type_num);
        buffer.read_u16()?;

        let ttl = buffer.read_u32()?;
        let data_length = buffer.read_u16()?;

        match query_type {
            QueryType::A => {
                let raw_address = buffer.read_u32()?;
                let address = Ipv4Addr::new(
                    get_nth_octal(raw_address, 1),
                    get_nth_octal(raw_address, 2),
                    get_nth_octal(raw_address, 3),
                    get_nth_octal(raw_address, 4),
                );
                Ok(DnsRecord::A {
                    domain,
                    address,
                    ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
                buffer.advance(data_length.into())?;
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

    pub fn write(&self, buffer: &mut WrappedBuffer) -> Result<usize, String> {
        let start_position = buffer.pos();

        match *self {
            DnsRecord::A {
                ref domain,
                ref address,
                ttl,
            } => {
                QueryName::write(buffer, &domain)?;
                buffer.write_u16(QueryType::A.to_u16())?;
                buffer.write_u16(1)?; // "class" (always 1)
                buffer.write_u32(ttl)?;
                buffer.write_u16(4)?; // length of data specific to this record type - in this case a 4-byte IP address

                let ip_addr_octets = address.octets();

                for octet in ip_addr_octets {
                    buffer.write_u8(octet)?;
                }
            }
            _ => (),
        };
        Ok(buffer.pos() - start_position)
    }
}

#[cfg(test)]
mod tests {
    use super::DnsRecord;
    use crate::parser::{
        dns_question::DnsQuestion,
        test_helpers::{are_same_enum_variant, get_buffer_at_question_section, GOOGLE_QUERY},
        wrapped_buffer::WrappedBuffer,
    };
    use std::{error::Error, net::Ipv4Addr};

    #[test]
    fn can_read_record_of_known_type() -> Result<(), Box<dyn Error>> {
        let mut buffer = get_buffer_after_question_section(String::from(GOOGLE_QUERY))?;
        let record = DnsRecord::read(&mut buffer)?;

        let expected_record = DnsRecord::A {
            domain: String::new(),
            address: Ipv4Addr::UNSPECIFIED,
            ttl: 0,
        };

        assert!(are_same_enum_variant(&record, &expected_record));
        Ok(())
    }

    #[test]
    fn can_write_record_of_known_type() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = "google.com";
        let expected_ip_address = Ipv4Addr::new(172, 29, 48, 1);
        let expected_ttl = 291;

        let record = DnsRecord::A {
            domain: String::from(expected_domain_name),
            address: expected_ip_address,
            ttl: expected_ttl,
        };
        let mut buffer = WrappedBuffer::new();
        record.write(&mut buffer)?;
        buffer.seek(0)?;

        match DnsRecord::read(&mut buffer)? {
            DnsRecord::A {
                domain,
                address,
                ttl,
            } => {
                assert_eq!(domain, expected_domain_name);
                assert_eq!(address, expected_ip_address);
                assert_eq!(ttl, expected_ttl);
            }
            _ => panic!(
                "Expected to read back a DNS record of the same type as the record that was written."
            ),
        }

        Ok(())
    }

    #[test]
    #[ignore = "Need to edit a packet to have an unrecognised query type"]
    fn can_read_record_of_unknown_type() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    #[test]
    fn skips_write_for_record_of_unknown_type() -> Result<(), Box<dyn Error>> {
        let unknown_record = DnsRecord::UNKNOWN {
            domain: String::from("google.com"),
            query_type: 10,
            data_length: 22,
            ttl: 8541,
        };
        let mut buffer = WrappedBuffer::new();
        unknown_record.write(&mut buffer)?;
        buffer.seek(0)?;

        match DnsRecord::read(&mut buffer)? {
            DnsRecord::UNKNOWN {
                domain,
                query_type,
                data_length,
                ttl,
            } => {
                assert_eq!(domain, String::new());
                assert_eq!(query_type, 0);
                assert_eq!(data_length, 0);
                assert_eq!(ttl, 0);
            }
            _ => panic!("Expected to skip writing DNS record of unknown type."),
        }
        Ok(())
    }

    #[test]
    fn reads_domain_name_successfully() -> Result<(), Box<dyn Error>> {
        let expected_domain_name = String::from("google.com");
        let mut buffer = get_buffer_after_question_section(String::from(GOOGLE_QUERY))?;

        match DnsRecord::read(&mut buffer)? {
            DnsRecord::A { domain, .. } => assert_eq!(domain, expected_domain_name),
            _ => panic!("Expected to receive a known record type."),
        };
        Ok(())
    }

    #[test]
    fn reads_ip_address_successfully() -> Result<(), Box<dyn Error>> {
        let expected_ip = Ipv4Addr::new(142, 250, 71, 78);
        let mut buffer = get_buffer_after_question_section(String::from(GOOGLE_QUERY))?;

        match DnsRecord::read(&mut buffer)? {
            DnsRecord::A { address, .. } => assert_eq!(address, expected_ip),
            _ => panic!("Expected to receive a known record type."),
        };
        Ok(())
    }

    #[test]
    fn reads_ttl_successfully() -> Result<(), Box<dyn Error>> {
        let expected_time_to_live = 265;
        let mut buffer = get_buffer_after_question_section(String::from(GOOGLE_QUERY))?;

        match DnsRecord::read(&mut buffer)? {
            DnsRecord::A { ttl, .. } => assert_eq!(ttl, expected_time_to_live),
            _ => panic!("Expected to receive a known record type."),
        };
        Ok(())
    }

    fn get_buffer_after_question_section(
        input_file: String,
    ) -> Result<WrappedBuffer, Box<dyn Error>> {
        let mut buffer = get_buffer_at_question_section(input_file)?;
        DnsQuestion::read(&mut buffer)?;
        Ok(buffer)
    }
}
