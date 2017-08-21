
use std::env;

pub const KAFKA_HOSTS: Option<String> = env::var("KAFKA_HOSTS").ok();

pub const KAFKA_SSL_KEY: Option<String> = env::var("KAFKA_SSL_KEY").ok();
pub const KAFKA_SSL_CERT: Option<String> = env::var("KAFKA_SSL_CERT").ok();
