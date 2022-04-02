const DEFAULT_REDIS_HOST: &str = "192.168.1.188:6379";
const DEFAULT_REDIS_PORT: &str = "6369";

pub fn get_redis_host() -> String {
    option_env!("REDIS_HOST")
        .unwrap_or(DEFAULT_REDIS_HOST)
        .to_string()
}

pub fn get_redis_port() -> String {
    option_env!("REDIS_PORT")
        .unwrap_or(DEFAULT_REDIS_PORT)
        .to_string()
}

pub fn get_connection_string() -> String {
    format!("{}:{}", get_redis_host(), get_redis_port())
}
