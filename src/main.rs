// Imports
mod config;
use config::{RECEIVERS, SENDERS};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use std::error::Error;

// Constants
const GREEN: &'static str = "\x1b[32;1m";
const NC: &'static str = "\x1b[0m";

// Async main
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut tasks = Vec::new();
        for sender in SENDERS.iter() {
            let task = tokio::spawn(async move {
                let msg = MessageBuilder::new()
                .from((sender.messenger, sender.creds.mail_addr))
                .to(RECEIVERS.to_vec())
                .subject("Mail vom MOT")
                .text_body("Shid 💩")
                .html_body(r#"
                    <img src="https://www.pngall.com/wp-content/uploads/2016/03/Troll-Face-Meme-PNG.png">
                    "#);
                SmtpClientBuilder::new("smtp.gmail.com", 587)
                    .implicit_tls(false)
                    .credentials((sender.creds.mail_addr, sender.creds.passwd))
                    .connect()
                    .await
                    .expect("Unable to connect to Gmail smtp server")
                    .send(msg)
                    .await
                    .expect("Unable to send message");
                println!(
                    "{}INFO{} Sending email from {}",
                    GREEN, NC, sender.creds.mail_addr
                );
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await?;
        }
    }
}
