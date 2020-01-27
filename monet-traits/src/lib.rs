pub trait Currency<'a> {
    const UNITS: u8;
    const CODE: &'a str;
    const NAME: &'a str;
}
