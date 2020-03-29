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

    pub type JoyConResult<T> = Result<T, JoyConError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
