use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailClient {
    transport: SmtpTransport,
    origin: String,
}

impl EmailClient {
    pub fn new() -> EmailClient {
        let creds = Credentials::new("771768137@qq.com".to_owned(), "pzpxdaqnuggobegg".to_owned());

        let transport = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .credentials(creds)
            .port(465)
            .build();

        EmailClient {
            transport: transport,
            origin: "771768137@qq.com".to_owned(),
        }
    }

    pub fn send(&self, destination: &str, subject: &str, body: String) {
        let email = Message::builder()
            .from(self.origin.parse().unwrap())
            .to(destination.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        match self.transport.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
