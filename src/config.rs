use lazy_static::lazy_static;
//////// CONFIG START ////////
lazy_static! {
    /// List of sending mails
    pub static ref SENDERS: Vec<Sender<'static>> = vec![
        /* Sender::new(
            Credentials::new("your.sending.email@mgail.com", "your google password"),
            "Title of the email"
        ), */
    ];
    /// EMail addrs on this list will all get sent any email from one of the SENDERS: Example
    pub static ref RECEIVERS: &'static[&'static str] = &[
        // "your.target@email.example.com",
        // "your.other.target@email.example.com",
    ];
}

//////// CONFIG END ////////
pub struct Sender<'a> {
    pub creds: Credentials<'a>,
    pub messenger: &'a str,
}

impl<'a> Sender<'a> {
    pub const fn new(creds: Credentials<'a>, messenger: &'a str) -> Self {
        Self { creds, messenger }
    }
}

/// Contains credentials of the sender to access gmail
pub struct Credentials<'a> {
    pub mail_addr: &'a str,
    pub passwd: &'a str,
}
impl<'a> Credentials<'a> {
    pub const fn new(mail_addr: &'a str, passwd: &'a str) -> Self {
        Self { mail_addr, passwd }
    }
}
