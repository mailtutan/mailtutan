use anyhow::{Context, Result};
use chrono::Local;
use mail_parser;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Default, Clone)]
pub struct Message {
    pub id: Option<usize>,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub created_at: Option<String>,
    pub attachments: Vec<Attachment>,
    #[serde(skip_serializing)]
    pub source: Vec<u8>,
    pub formats: Vec<String>,
    #[serde(skip_serializing)]
    pub html: Option<String>,
    #[serde(skip_serializing)]
    pub plain: Option<String>,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct MessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: Message,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct Attachment {
    pub cid: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub filename: String,
    #[serde(skip_serializing)]
    pub body: Vec<u8>,
}

impl Message {
    pub fn from(data: &Vec<u8>) -> Result<Self> {
        use mail_parser::HeaderValue;

        let message = mail_parser::Message::parse(data.as_ref()).context("parse message")?;

        let sender = {
            if let HeaderValue::Address(addr) = message.from() {
                format!(
                    "{} {}",
                    addr.name.as_ref().context("parse sender name")?,
                    addr.address.as_ref().context("parse sender address")?
                )
            } else {
                "".to_owned()
            }
        };

        let recipients = {
            let mut list: Vec<String> = vec![];

            if let HeaderValue::Address(addr) = message.to() {
                list.push(format!(
                    "{}",
                    addr.address.as_ref().context("parse recipient address")?
                ));
            }

            list
        };
        let subject = message.subject().unwrap_or("").to_string();

        let mut formats = vec!["source".to_owned()];
        let mut html: Option<String> = None;
        let mut plain: Option<String> = None;

        if message.html_body_count() > 0 {
            formats.push("html".to_owned());
            html = Some(message.body_html(0).unwrap().to_string());
        }

        if message.text_body_count() > 0 {
            formats.push("plain".to_owned());
            plain = Some(message.body_text(0).unwrap().to_string());
        }

        use mail_parser::MimeHeaders;

        let attachments = message
            .attachments()
            .map(|attachment| Attachment {
                filename: attachment
                    .attachment_name()
                    .unwrap_or("unknown")
                    .to_string(),
                file_type: attachment.content_type().unwrap().ctype().to_string(),
                body: attachment.contents().to_vec(),
                cid: Uuid::new_v4().to_string(),
            })
            .collect();

        Ok(Self {
            id: None,
            sender,
            recipients,
            subject,
            created_at: Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            attachments,
            source: data.to_owned(),
            formats,
            html,
            plain,
        })
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

        let message = Message::from(&data).unwrap();
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

        let message = Message::from(&data).unwrap();
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

        let message = Message::from(&data).unwrap();
        assert_eq!(message.subject, "");
    }
}
