use mailin_embedded::{Handler, Server, SslConfig};
use mailparse::*;
use std::io;

#[derive(Clone)]
struct MyHandler {
    pub data: Vec<u8>,
}

impl Handler for MyHandler {
    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        self.data.append(&mut buf.to_owned());

        Ok(())
    }

    fn data_end(&mut self) -> mailin_embedded::Response {
        dbg!("we got the data");
        // dbg!(&self.data.to_);
        // dbg!(std::str::from_utf8(&self.data));
        // SmtpParser.parse(&self.data, &SmtpContext::default());

        let parsed = parse_mail(&self.data).unwrap();
        dbg!(parsed.headers.get_first_value("Subject"));
        dbg!(parsed.subparts.len());
        dbg!(parsed.get_body());

        mailin_embedded::response::OK
    }
}

pub async fn serve() {
    let handler = MyHandler { data: vec![] };
    let mut server = Server::new(handler);

    server
        .with_name("example.com")
        .with_ssl(SslConfig::None)
        .unwrap()
        .with_addr("127.0.0.1:2525")
        .unwrap();

    server.serve().unwrap();
}
