pub mod joycon;

pub mod prelude {
    pub use hidapi::*;
    pub use crate::result::*;
}

pub mod result {
    use hidapi::HidError;

    #[derive(Debug)]
    pub enum JoyConError {
        HidApiError(hidapi::HidError),
        JoyConDeviceError(JoyConDeviceError),
        JoyConReportError(JoyConReportError),
    }

    impl From<hidapi::HidError> for JoyConError {
        fn from(e: HidError) -> Self {
            JoyConError::HidApiError(e)
        }
    }

    #[derive(Debug)]
    pub enum JoyConDeviceError {
        InvalidVendorID(u16),
        InvalidProductID(u16),
    }

    impl From<JoyConDeviceError> for JoyConError {
        fn from(e: JoyConDeviceError) -> Self {
            JoyConError::JoyConDeviceError(e)
        }
    }

    #[derive(Debug)]
    pub enum JoyConReportError {
        InvalidSimpleHIDReport(Vec<u8>),
        InvalidStandardFullReport(InvalidStandardFullReport),
    }

    impl From<JoyConReportError> for JoyConError {
        fn from(e: JoyConReportError) -> Self {
            JoyConError::JoyConReportError(e)
        }
    }

    #[derive(Debug)]
    pub enum InvalidStandardFullReport {
        InvalidReport(Vec<u8>),
        Battery(u8),
        ConnectionInfo(u8),
        InvalidInputReportId(u8),
    }

    impl From<InvalidStandardFullReport> for JoyConReportError {
        fn from(e: InvalidStandardFullReport) -> Self {
            JoyConReportError::InvalidStandardFullReport(e)
        }
    }

    impl From<InvalidStandardFullReport> for JoyConError {
        fn from(e: InvalidStandardFullReport) -> Self {
            let report_error = JoyConReportError::from(e);
            JoyConError::from(report_error)
        }
    }

    pub type JoyConResult<T> = Result<T, JoyConError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
