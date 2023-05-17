use std::todo;

use chrono::Utc;
use mailparse::*;
use serde::Serialize;

#[derive(Serialize, Debug, Default, Clone)]
pub struct Message {
    pub id: Option<usize>,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub created_at: Option<String>,
    pub attachments: Vec<String>,
    pub source: Vec<u8>,
    pub formats: Vec<String>,
    pub html: Option<String>,
    pub plain: Option<String>,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct MessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: Message,
}

impl From<&Vec<u8>> for Message {
    fn from(data: &Vec<u8>) -> Self {
        let parsed = parse_mail(data.as_ref()).unwrap();

        let mut formats = vec![];
        let mut html: Option<String> = None;
        let mut plain: Option<String> = None;

        for part in parsed.subparts {
            match part.ctype.mimetype.as_ref() {
                "text/html" => {
                    formats.push("html".to_owned());
                    html = part.get_body().ok();
                }
                "text/plain" => {
                    formats.push("plain".to_owned());
                    plain = part.get_body().ok();
                }
                _ => todo!(),
            }
        }

        Self {
            id: None,
            sender: parsed.headers.get_first_value("From").unwrap_or_default(),
            recipients: parsed.headers.get_all_values("To"),
            subject: parsed
                .headers
                .get_first_value("Subject")
                .unwrap_or_default(),
            created_at: Some(Utc::now().to_rfc3339()),
            attachments: vec![],
            source: data.to_owned(),
            formats: formats,
            html: html,
            plain: plain,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subject() {
        let data = concat!(
            "From: Private Person <me@fromdomain.com>\n",
            "To: A Test User <test@todomain.com>\n",
            "Subject: SMTP e-mail test\n",
            "\n",
            "This is a test e-mail message.\n"
        )
        .as_bytes()
        .to_vec();

        let message = Message::from(&data);
        assert_eq!(message.subject, "SMTP e-mail test");
    }

    #[test]
    fn test_felan() {
        let data = concat!(
            "Subject: This is a test email\n",
            "Content-Type: multipart/alternative; boundary=foobar\n",
            "Date: Sun, 02 Oct 2016 07:06:22 -0700 (PDT)\n",
            "\n",
            "--foobar\n",
            "Content-Type: text/plain; charset=utf-8\n",
            "Content-Transfer-Encoding: quoted-printable\n",
            "\n",
            "This is the plaintext version, in utf-8. Proof by Euro: =E2=82=AC\n",
            "--foobar\n",
            "Content-Type: text/html\n",
            "Content-Transfer-Encoding: base64\n",
            "\n",
            "PGh0bWw+PGJvZHk+VGhpcyBpcyB0aGUgPGI+SFRNTDwvYj4gdmVyc2lvbiwgaW4g \n",
            "dXMtYXNjaWkuIFByb29mIGJ5IEV1cm86ICZldXJvOzwvYm9keT48L2h0bWw+Cg== \n",
            "--foobar--\n",
            "After the final boundary stuff gets ignored.\n"
        )
        .as_bytes()
        .to_vec();

        let message = Message::from(&data);
        assert_eq!(message.subject, "This is a test email");
    }

    #[test]
    fn test_subject_is_not_found() {
        let data = concat!(
            "Content-Type: multipart/alternative; boundary=foobar\n",
            "Date: Sun, 02 Oct 2016 07:06:22 -0700 (PDT)\n",
            "\n",
            "--foobar\n",
            "Content-Type: text/plain; charset=utf-8\n",
            "Content-Transfer-Encoding: quoted-printable\n",
            "\n",
            "This is the plaintext version, in utf-8. Proof by Euro: =E2=82=AC\n",
            "--foobar\n",
            "Content-Type: text/html\n",
            "Content-Transfer-Encoding: base64\n",
            "\n",
            "PGh0bWw+PGJvZHk+VGhpcyBpcyB0aGUgPGI+SFRNTDwvYj4gdmVyc2lvbiwgaW4g \n",
            "dXMtYXNjaWkuIFByb29mIGJ5IEV1cm86ICZldXJvOzwvYm9keT48L2h0bWw+Cg== \n",
            "--foobar--\n",
            "After the final boundary stuff gets ignored.\n"
        )
        .as_bytes()
        .to_vec();

        let message = Message::from(&data);
        assert_eq!(message.subject, "");
    }
}
