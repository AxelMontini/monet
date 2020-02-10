pub trait Currency {
    const UNITS: u8;
    const CODE: &'static str;
    const NAME: &'static str;
}
