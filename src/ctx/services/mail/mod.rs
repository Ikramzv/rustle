use lettre::{
    SmtpTransport, Transport,
    address::AddressError,
    message::{Message, header::ContentType},
};

use crate::config::CONFIG;

fn send_mail(mailer: &SmtpTransport, m: Message) -> Result<(), String> {
    let mailer = mailer.clone();
    let m = m.clone();

    mailer.send(&m).map_err(|e| e.to_string())?;
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

    let mailer = mailer.clone();

    tokio::task::spawn(async move { send_mail(&mailer, m) })
        .await
        .map_err(|e| e.to_string())??;

    Ok(())
}
