use clap::Parser;
use mailtutan_lib::{storage::Memory, Mailtutan};
use std::net::Ipv4Addr;
use tokio::sync::broadcast;

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

    pub fn build(&self) -> Mailtutan {
        Mailtutan {
            ip: self.ip,
            http_port: self.http_port,
            smtp_port: self.smtp_port,
            storage: Box::new(Memory::new()),
            ws_sender: broadcast::channel(100).0,
        }
    }
}
