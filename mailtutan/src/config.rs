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

    /// HTTP Auth
    #[arg(
        long = "http-auth",
        env("MAILTUTAN_HTTP_AUTH"),
        default_value_t = false
    )]
    pub http_auth: bool,

    /// HTTP Username
    #[arg(
        long = "http-username",
        env("MAILTUTAN_HTTP_USERNAME"),
        default_value = "admin"
    )]
    pub http_username: String,

    /// HTTP Password
    #[arg(
        long = "http-password",
        env("MAILTUTAN_HTTP_PASSWORD"),
        default_value = "admin"
    )]
    pub http_password: String,

    /// Messages Limit
    #[arg(
        long = "messages-limit",
        env("MAILTUTAN_MESSAGES_LIMIT"),
        default_value_t = 1000
    )]
    pub messages_limit: usize,
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
            http_username: self.http_username.clone(),
            http_password: self.http_password.clone(),
            http_auth: self.http_auth,
            storage: Box::new(Memory::new(self.messages_limit)),
            ws_sender: broadcast::channel(100).0,
            messages_limit: self.messages_limit,
        }
    }
}
