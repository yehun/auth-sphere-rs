
pub struct RedisServiceOpt {
    pub host: String,
    pub port: u16,
    pub db: Option<u8>,
    pub password: Option<String>,
    pub timeout: Option<f32>
}