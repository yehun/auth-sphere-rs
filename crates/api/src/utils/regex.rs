
lazy_static::lazy_static! {
    pub static ref PHONE_REGEX: regex::Regex = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
    pub static ref EMAIL_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

pub fn is_phone(phone: &str) -> bool {
    PHONE_REGEX.is_match(phone)
}

 pub fn is_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}
