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
}