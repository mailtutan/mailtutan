# Mailtutan

This is a repository for Mailtutan, a SMTP server built in Rust. The server allows you to simulate an SMTP (Simple Mail Transfer Protocol) server for testing and development purposes without actually sending emails to real addresses.

![screenshot-1](https://raw.githubusercontent.com/mailtutan/mailtutan/main/screenshot-1.jpg)

## Features

- Simulates an SMTP server locally for testing and development.
- Logs all incoming emails, including the headers and body.
- Provides a web interface and REST API to view the list of emails.
- Supports multiple client connections simultaneously.
- Supports real-time updates using WebSockets for the web interface.
- Lightweight and efficient implementation in Rust.

## Installation

1. Clone this repository to your local machine using the following command:

   ```
   git clone https://github.com/your-username/mailtutan.git
   ```

2. Change to the cloned directory:

   ```
   cd mailtutan
   ```

3. Build the project using Cargo:

   ```
   cargo build --release
   ```

4. Run the server:

   ```
   cargo run --release
   ```

   The server will start listening on the default SMTP port (1025) on your local machine.

## Configuration

By default, Mailtutan listens on port 1025 and logs incoming emails to memory. However, you can customize the server configuration by supported parameters and environment variables

## Usage

To send emails to Mailtutan, configure your email client or application to use the server's IP address or hostname and the configured port number. Any emails sent to this server will be captured and logged.

You can access the web interface to view the list of logged emails by navigating to `http://localhost:1080` in your web browser. Additionally, a REST API is available at `http://localhost:1080/api/messages` to programmatically access the email data.

## Contributing

Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/mailtutan/mailtutan). Make sure to follow the existing coding style and guidelines.

## License

This project is licensed under the [MIT License](LICENSE.txt). Feel free to use, modify, and distribute it as per the terms of the license.

## Acknowledgments

- This project was inspired by the need for a lightweight fake SMTP server for testing and development purposes.
- Thanks to the Rust programming language community for providing excellent tools and libraries for building efficient and reliable software.

## Contact

If you have any questions or need further assistance, feel free to contact the project maintainer at mohsen@alizadeh.us.
