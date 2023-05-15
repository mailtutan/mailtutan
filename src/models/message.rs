use mailparse::*;
use serde::Serialize;
use std::time::SystemTime;

// id
// sender
// recipients
// subject
// size
// type: "text/plan"
// created_at
// formats: source,plain, html
// attachments
#[derive(Serialize, Debug, Default, Clone)]
pub struct Message {
    pub id: Option<usize>,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub created_at: Option<SystemTime>,
    pub attachments: Vec<String>,
}

impl From<&Vec<u8>> for Message {
    fn from(data: &Vec<u8>) -> Self {
        let parsed = parse_mail(data.as_ref()).unwrap();

        // dbg!(&parsed);

        Self {
            id: None,
            sender: "".to_owned(),
            recipients: vec![],
            subject: parsed
                .headers
                .get_first_value("Subject")
                .unwrap_or_default(),
            created_at: None,
            attachments: vec![],
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
