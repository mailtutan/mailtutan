# Mailtutan

This is a repository for Mailtutan, a SMTP server built in Rust. The server allows you to simulate an SMTP (Simple Mail Transfer Protocol) server for testing and development purposes without actually sending emails to real addresses.

![screenshot-1](https://raw.githubusercontent.com/mailtutan/mailtutan/main/screenshot-1.jpg)

## Features

- Simulates an SMTP server locally for testing and development.
- Logs all incoming emails, including the headers and body.
- Provides a web interface and REST API to view the list of emails.
- Basic HTTP Authentication on web interface and REST API ( rfc7617 )
- Supports multiple client connections simultaneously.
- Supports encryption on transit (TLS) on SMTP
- Supports real-time updates using WebSockets for the web interface.
- Lightweight and efficient implementation in Rust. The docker image on docker hub is < 2MB.
- Multiple storage options: Memory ( default ), Maildir ( in file )
- Supports SMTP Auth ( Plain )

## Installation

#### Compile from source
```
git clone git@github.com:mailtutan/mailtutan.git
cd mailtutan
make all # app will be available at ./target/release/mailtutan
```

#### Cargo
```
cargo install mailtutan
```

#### AUR
```
yay -S mailtutan-bin
```

#### Docker
```
docker run -p 1080:1080 -p 1025:1025 -d mailtutan/mailtutan:latest
```

#### Download binary executables
```
Check out the latest release
```

## Configuration

By default, Mailtutan listens on port 1025 for SMTP and logs incoming emails to memory. The web interface is available on port 1080. However, you can customize the server configuration by supported parameters and environment variables.
```
$ mailtutan -h
An SMTP server for test and development environments.

Usage: mailtutan [OPTIONS]

Options:
      --ip <IP>
          IPv4 address [env: MAILTUTAN_IPADDR=] [default: 0.0.0.0]
      --http-port <HTTP_PORT>
          HTTP Port number [env: MAILTUTAN_HTTP_PORT=] [default: 1080]
      --smtp-port <SMTP_PORT>
          SMTP Port number [env: MAILTUTAN_SMTP_PORT=] [default: 1025]
      --smtp-cert-path <SMTP_CERT_PATH>
          SMTP Cert Path [env: MAILTUTAN_SMTP_CERT_PATH=]
      --smtp-key-path <SMTP_KEY_PATH>
          SMTP Key Path [env: MAILTUTAN_SMTP_KEY_PATH=]
      --smtp-auth-username <SMTP_AUTH_USERNAME>
          SMTP Auth Username [env: MAILTUTAN_AUTH_USERNAME=]
      --smtp-auth-password <SMTP_AUTH_PASSWORD>
          SMTP Auth Password [env: MAILTUTAN_AUTH_PASSWORD=]
      --http-auth
          HTTP Auth [env: MAILTUTAN_HTTP_AUTH=]
      --http-username <HTTP_USERNAME>
          HTTP Username [env: MAILTUTAN_HTTP_USERNAME=] [default: admin]
      --http-password <HTTP_PASSWORD>
          HTTP Password [env: MAILTUTAN_HTTP_PASSWORD=] [default: admin]
      --messages-limit <MESSAGES_LIMIT>
          Messages Limit [env: MAILTUTAN_MESSAGES_LIMIT=] [default: 1000]
      --storage <STORAGE>
          Storage [env: MAILTUTAN_STORAGE=] [default: memory] [possible values: memory, maildir]
      --maildir-path <MAILDIR_PATH>
          Storage [env: MAILTUTAN_MAILDIR_PATH=] [default: maildir]
  -h, --help
          Print help
  -V, --version
          Print version

```


## Usage

To send emails to Mailtutan, configure your email client or application to use the server's IP address or hostname and the configured port number. Any emails sent to this server will be captured and logged.

You can access the web interface to view the list of logged emails by navigating to `http://localhost:1080` in your web browser. Additionally, a REST API is available at `http://localhost:1080/api/` to programmatically access the email data.

## API
|HTTP method | endpoint | description|
|---|---|---|
|GET|/api/messages| get list of messages|
|GET|/api/messages/:id/json| get message's details|
|GET|/api/messages/:id/source| get message's source|
|GET|/api/messages/:id/plain| get message's plain text|
|GET|/api/messages/:id/html| get message's HTML|
|GET|/api/messages/:id/parts/:cid| get message's attachment|
|DELETE|/api/messages/:id| delete a message|
|DELETE|/api/messages| delete all messages|
|GET|/api/version| get application version|
|GET|/api/ws| subscribe to web socket to get updates|


## Contributing

Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/mailtutan/mailtutan). Make sure to follow the existing coding style and guidelines.

## License

This project is licensed under the [MIT License](LICENSE.txt). Feel free to use, modify, and distribute it as per the terms of the license.

## Acknowledgments

- This project was inspired by the need for a lightweight fake SMTP server for testing and development purposes.
- Thanks to the Rust programming language community for providing excellent tools and libraries for building efficient and reliable software.
