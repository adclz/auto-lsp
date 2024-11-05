pub trait Key {
    fn get_key<'a>(&self, source_code: &'a [u8]) -> &'a str;
}
