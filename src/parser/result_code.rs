#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl ResultCode {
    pub fn from_number(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResultCode;

    #[test]
    pub fn creates_noerr_result_from_zero() {
        assert_eq!(ResultCode::from_number(0), ResultCode::NOERROR);
    }

    #[test]
    pub fn creates_noerr_result_from_num_gt_five() {
        assert_eq!(ResultCode::from_number(6), ResultCode::NOERROR);
        assert_eq!(ResultCode::from_number(0), ResultCode::from_number(6));
    }
}
