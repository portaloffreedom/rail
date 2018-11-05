use imap;
use rail_imap::Connection;

enum FolderStatus {
    ///The number of messages in the mailbox.
    Messages,
    ///The number of messages with the \Recent flag set.
    Recent, 
    /// The next unique identifier value of the mailbox.
    UIDNext, 
    /// The unique identifier validity value of the mailbox.
    UIDValidity, 
    /// The number of messages which do not have the \Seen flag set.
    Unseen,
}

impl From<FolderStatus> for &'static str {
    fn from(status: FolderStatus) -> &'static str
    {
        match status {
            Messages => "MESSAGES",
            Recent   => "RECENT",
            UIDNext  => "UIDNEXT",
            UIDValidity => "UIDVALIDITY",
            Unseen   => "UNSEEN",
        }
    }
} 

pub struct Folder<'a> {
    connection: &'a mut Box<Connection>,
    name: &'a str,
    n_messages: u32,
    n_unseen: u32,
}

impl<'a> Folder<'a> {
    pub fn new(connection: &'a mut Box<Connection>, name: &'a imap::Name)
    -> Self
    {
        let mut folder = Self {
            connection,
            name: name.name(),
            n_messages: 0,
            n_unseen: 0,
        };

        if let Err(e) = folder.update() { 
            println!("Error dowloading status for folder {}: {}", folder.name, e);
        }
        
        folder
    }

    pub fn update(&mut self)
    -> imap::error::Result<()>
    {
        // let r = self.connection.status(self.name, "(MESSAGES RECENT UIDNEXT UIDVALIDITY UNSEEN)")?;
        let r = self.connection.status(self.name, "(MESSAGES RECENT UNSEEN)")?;
        self.n_messages = r.exists;
        self.n_unseen = r.unseen.unwrap_or(0);
        // self.n_recent = r.recent;
        Ok(())
    }
}