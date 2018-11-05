use imap::client::Session;
use std::io::{Read, Write};
use imap::error::Result as ImapResult;
use imap;
use native_tls::TlsStream;
use std::net::TcpStream;

pub trait Connection {
    fn list_folders(&mut self) 
    -> ImapResult<imap::ZeroCopy<std::vec::Vec<imap::Name>>>
    {
        self.list("", "*")
    }
    
    fn list_top_folders(&mut self)
    -> ImapResult<imap::ZeroCopy<std::vec::Vec<imap::Name>>>
    {
        self.list("", "%")
    }

    fn list(&mut self, base_folder: &str, search_pattern: &str)
    -> ImapResult<imap::ZeroCopy<std::vec::Vec<imap::Name>>>;

    fn status(&mut self, mailbox_name: &str, status_data_items: &str)
    -> ImapResult<imap::Mailbox>;
}

// TLS connection -------------------------------------------------------------
pub struct TLSConnection {
    socket: Session<TlsStream<TcpStream>>
}

impl TLSConnection {
    pub fn new(socket: Session<TlsStream<TcpStream>>) -> Self {
        Self {
            socket,
        }
    }
}

impl Connection for TLSConnection {
    fn list(&mut self, base_folder: &str, search_pattern: &str)
    -> ImapResult<imap::ZeroCopy<std::vec::Vec<imap::Name>>>
    {
        self.socket.list(base_folder, search_pattern)
    }

    ///TODO this sucks
    fn status(&mut self, mailbox_name: &str, status_data_items: &str)
    -> ImapResult<imap::Mailbox>
    {
        let mailbox_name: String = if mailbox_name.contains(' ') {
            format!("\"{}\"", mailbox_name)
        } else {
            String::from(mailbox_name)
        };

        // self.socket.status(mailbox_name, status_data_items)
        let command = format!("STATUS {} (MESSAGES UNSEEN RECENT)", mailbox_name);
        let r = self.socket.run_command_and_read_response(command.as_ref())?;

        println!("RAW RESULT '{}'", String::from_utf8_lossy(&r));
        //* STATUS Bozze (MESSAGES 2 RECENT 0 UNSEEN 0)\n

        let template_answer = format!("* STATUS {} (MESSAGES # RECENT # UNSEEN #)\n", mailbox_name);
        let mut iter_source = r.iter();
        let mut iter_target = template_answer.bytes();
        
        let mut messages: Option<u32> = None;
        let mut recent:   Option<u32> = None;
        let mut unseen:   Option<u32> = None;

        while let Some(c_target) = iter_target.next() {
            if c_target == '#' as u8 {
                // parse number
                let mut number_s = Vec::new();
                while let Some(c_source) = iter_source.next() {
                    if *c_source < '0' as u8 || *c_source > '9' as u8 {
                        let c_target = iter_target.next().unwrap();
                        if c_target != *c_source {
                            return Err(imap::error::Error::Parse(
                                imap::error::ParseError::Invalid(r.clone())
                            ));
                        }
                        break;
                    }
                    number_s.push(*c_source);
                }
                let number: u32 = String::from_utf8_lossy(&number_s).parse().unwrap();
                
                // save number into variable
                if messages.is_none() {
                    messages = Some(number);
                } else if recent.is_none() {
                    recent = Some(number);
                } else if unseen.is_none() {
                    unseen = Some(number);
                } else {
                    unreachable!();
                }

            } else if let Some(c_source) = iter_source.next() {
                
                // finished?
                if *c_source == '\n' as u8 || *c_source == '\r' as u8 {
                    if c_target == '\n' as u8 {
                        break;
                    }
                }

                // continue?
                if *c_source != c_target {
                    return Err(imap::error::Error::Parse(
                        imap::error::ParseError::Invalid(r.clone())
                    ));
                }
            } else {
                return Err(imap::error::Error::Parse(
                    imap::error::ParseError::Invalid(r.clone())
                ));
            }
        }

        Ok(imap::Mailbox {
            exists: messages.unwrap_or(0),
            unseen: unseen,
            recent: recent.unwrap_or(0),
            .. imap::Mailbox::default()
        })
    }
}

// Insecure connection --------------------------------------------------------
pub struct InsecureConnection {
    socket: Session<TcpStream>
}

impl InsecureConnection {
    pub fn new(socket: Session<TcpStream>) -> Self {
        Self {
            socket,
        }
    }
}

impl Connection for InsecureConnection {
    fn list(&mut self, base_folder: &str, search_pattern: &str)
    -> ImapResult<imap::ZeroCopy<std::vec::Vec<imap::Name>>>
    {
        self.socket.list(base_folder, search_pattern)
    }

    fn status(&mut self, mailbox_name: &str, status_data_items: &str)
    -> ImapResult<imap::Mailbox>
    {
        // self.socket.status(mailbox_name, status_data_items)
        let command = format!("STATUS {} (MESSAGES UNSEEN RECENT)", mailbox_name);
        let r = self.socket.run_command_and_read_response(command.as_ref())?;

        Ok(imap::Mailbox {
            exists: 1,
            unseen: Some(1),
            recent: 1,
            .. imap::Mailbox::default()
        })
    }
}