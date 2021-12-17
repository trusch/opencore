use super::errors::Error;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport, message::Mailbox};

pub struct Manager {
    transport: SmtpTransport,
}

impl Manager {
    pub fn new(host: &str, port: u16, username: &str, password: &str) -> Result<Self, Error> {
        let credentials = Credentials::new(username.into(), password.into());
        let host = format!("{}:{}", host, port);
        let transport = SmtpTransport::relay(&host)?
            .credentials(credentials)
            .build();
        Ok(Self { transport })
    }

    pub fn send_invite(&self, email: &str, name: &str) -> Result<(), Error> {
        let message = Message::builder()
            .from("opencore <noreply@opencore>".parse()?)
            .to(Mailbox { 
                name: Some(name.into()) , 
                email: email.parse::<lettre::Address>()?,
             })
            .subject("invitation to opencore")
            .body(String::from("Your are invited to join this opencore instance"))?;
        
        self.transport.send(&message)?;
        
        Ok(())
    }
}
