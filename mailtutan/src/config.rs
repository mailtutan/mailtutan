use clap::Parser;
use std::net::Ipv4Addr;

/// Mailtutan
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// IPv4 address
    #[arg(long = "ip", env("MAILTUTAN_IPADDR"), default_value = "0.0.0.0")]
    pub ip: Ipv4Addr,

    /// HTTP Port number
    #[arg(long = "http-port", env("MAILTUTAN_HTTP_PORT"), default_value_t = 1080)]
    pub http_port: u16,

    /// SMTP Port number
    #[arg(long = "smtp-port", env("MAILTUTAN_SMTP_PORT"), default_value_t = 1025)]
    pub smtp_port: u16,
}

impl Config {
    pub fn from_env_and_args() -> Self {
        Self::parse()
    }

    pub fn get_api_uri(&self) -> String {
        format!("{}:{}", self.ip, self.http_port)
    }

    pub fn get_smtp_uri(&self) -> String {
        format!("{}:{}", self.ip, self.smtp_port)
    }
}
