use clap::Parser;
use clap::ValueEnum;
use std::net::Ipv4Addr;

#[derive(Clone, Debug, ValueEnum)]
pub enum StorageType {
    #[clap(value_enum)]
    Memory,
    #[clap(value_enum)]
    Maildir,
}

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

    /// SMTP Cert Path
    #[arg(long = "smtp-cert-path", env("MAILTUTAN_SMTP_CERT_PATH"))]
    pub smtp_cert_path: Option<String>,

    /// SMTP Key Path
    #[arg(long = "smtp-key-path", env("MAILTUTAN_SMTP_KEY_PATH"))]
    pub smtp_key_path: Option<String>,

    /// SMTP Auth Username
    #[arg(
        long = "smtp-auth-username",
        env("MAILTUTAN_AUTH_USERNAME"),
        requires = "smtp_key_path",
        requires = "smtp_cert_path"
    )]
    pub smtp_auth_username: Option<String>,

    /// SMTP Auth Password
    #[arg(
        long = "smtp-auth-password",
        env("MAILTUTAN_AUTH_PASSWORD"),
        requires = "smtp_key_path",
        requires = "smtp_cert_path"
    )]
    pub smtp_auth_password: Option<String>,

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

    /// Storage
    #[arg(long = "storage", env("MAILTUTAN_STORAGE"), default_value = "memory")]
    pub storage: StorageType,

    /// Storage
    #[arg(
        long = "maildir-path",
        env("MAILTUTAN_MAILDIR_PATH"),
        default_value = "maildir"
    )]
    pub maildir_path: String,

    /// Web Hook
    #[arg(long = "web-hook", env("MAILTUTAN_WEB_HOOK"))]
    pub web_hook: Option<String>,
}
