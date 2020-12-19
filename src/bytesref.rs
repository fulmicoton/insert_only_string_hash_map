/// `BytesRef` refers to a slice in the string data.
#[derive(Copy, Clone, Debug)]
pub struct BytesRef(pub(crate) u32);

impl BytesRef {
    pub fn is_null(&self) -> bool {
        self.0 == u32::max_value()
    }

    pub fn addr(&self) -> u32 {
        self.0
    }
}

impl Default for BytesRef {
    fn default() -> BytesRef {
        BytesRef(u32::max_value())
    }
}
