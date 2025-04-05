use lettre::{
    SmtpTransport, Transport,
    address::AddressError,
    message::{Message, header::ContentType},
};

use crate::config::CONFIG;

async fn send_mail(mailer: &SmtpTransport, m: &Message) -> Result<(), String> {
    let mailer = mailer.clone();
    let m = m.clone();

    tokio::task::spawn_blocking(move || mailer.send(&m))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn send_verification_mail(
    mailer: &SmtpTransport,
    email: String,
    code: String,
) -> Result<(), String> {
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

    send_mail(mailer, &m).await
}
