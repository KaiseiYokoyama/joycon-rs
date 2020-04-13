#![doc(html_logo_url = "https://user-images.githubusercontent.com/8509057/79100490-9a4a7900-7da1-11ea-9ee4-5e15439bbd0c.png")]

pub mod joycon;

pub mod prelude {
    pub use hidapi::*;
    pub use crate::result::*;
    pub use crate::joycon::*;
}

pub mod result {
    // use crate::prelude::SubCommand;
    use hidapi::HidError;

    #[derive(Debug)]
    pub enum JoyConError {
        HidApiError(hidapi::HidError),
        // SubCommandError(SubCommand),
        SubCommandError(u8),
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
        InvalidSimpleHidReport(InvalidSimpleHIDReport),
        InvalidStandardInputReport(InvalidStandardInputReport),
    }

    impl From<JoyConReportError> for JoyConError {
        fn from(e: JoyConReportError) -> Self {
            JoyConError::JoyConReportError(e)
        }
    }

    #[derive(Debug)]
    pub enum InvalidSimpleHIDReport {
        InvalidReport(Vec<u8>),
        InvalidStickDirection(u8),
    }

    impl From<InvalidSimpleHIDReport> for JoyConReportError {
        fn from(e: InvalidSimpleHIDReport) -> Self {
            JoyConReportError::InvalidSimpleHidReport(e)
        }
    }

    impl From<InvalidSimpleHIDReport> for JoyConError {
        fn from(e: InvalidSimpleHIDReport) -> Self {
            let report_error = JoyConReportError::from(e);
            JoyConError::from(report_error)
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
            JoyConReportError::InvalidStandardInputReport(e)
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
