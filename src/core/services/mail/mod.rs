use lettre::{
    SmtpTransport, Transport,
    address::AddressError,
    message::{Message, header::ContentType},
    transport::smtp::authentication::Credentials,
};

use crate::config::CONFIG;

pub struct MailService {
    smtp: SmtpTransport,
}

impl MailService {
    pub fn new() -> Self {
        let smtp = SmtpTransport::relay(&CONFIG.mail_config.host)
            .expect("SMTP Host not specified")
            .credentials(Credentials::new(
                CONFIG.mail_config.username.clone(),
                CONFIG.mail_config.password.clone(),
            ))
            .build();

        Self { smtp }
    }

    fn send_mail(mailer: &SmtpTransport, m: Message) -> Result<(), String> {
        let m = m.clone();

        mailer.send(&m).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn send_verification_mail(&self, email: String, code: String) -> Result<(), String> {
        let m = Message::builder()
            .from(
                CONFIG
                    .mail_config
                    .from
                    .parse()
                    .map_err(|e: AddressError| e.to_string())?,
            )
            .to(email.parse().map_err(|e: AddressError| e.to_string())?)
            .subject("Verification Code")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("Your verification code is {}", code))
            .map_err(|e| e.to_string())?;

        let mailer = self.smtp.clone();

        tokio::task::spawn(async move { MailService::send_mail(&mailer, m) })
            .await
            .map_err(|e| e.to_string())??;

        Ok(())
    }
}
