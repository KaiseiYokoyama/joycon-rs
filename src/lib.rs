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
        InvalidStandardFullReport(InvalidStandardInputReport),
    }

    impl From<JoyConReportError> for JoyConError {
        fn from(e: JoyConReportError) -> Self {
            JoyConError::JoyConReportError(e)
        }
    }

    #[derive(Debug)]
    pub enum InvalidStandardInputReport {
        InvalidReport(Vec<u8>),
        InvalidExtraReport(Vec<u8>),
        Battery(u8),
        ConnectionInfo(u8),
        InvalidInputReportId(u8),
    }

    impl From<InvalidStandardInputReport> for JoyConReportError {
        fn from(e: InvalidStandardInputReport) -> Self {
            JoyConReportError::InvalidStandardFullReport(e)
        }
    }

    impl From<InvalidStandardInputReport> for JoyConError {
        fn from(e: InvalidStandardInputReport) -> Self {
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
