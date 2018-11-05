extern crate native_tls;
extern crate imap;
#[macro_use]
extern crate qmlrs;
extern crate config;

pub mod rail_imap;

use imap::client;

// To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
// See: https://support.google.com/accounts/answer/6010255?hl=en
// Look at the `examples/gmail_oauth2.rs` for how to connect to gmail securely.
fn main2(username: String, password: String) -> Result<(), imap::error::Error> {
    let domain = "imap.gmail.com";
    let port = 993;
    let socket_addr = (domain, port);
    let ssl_connector = native_tls::TlsConnector::builder().build()?;
    let client = client::secure_connect(socket_addr, domain, &ssl_connector)?;

    let mut imap_socket = client.login(&username, &password)
        .map_err(|(error, _client)| error)?;

    println!("CONNECTION SUCCESSFUL {}\t SERVER {}:{}", username, domain, port);

    match imap_socket.capabilities() {
        Ok(capabilities) => {
            for capability in capabilities.iter() {
                println!("{}", capability);
            }
        }
        Err(e) => println!("Error parsing capabilities: {}", e),
    };

    match imap_socket.list("", "%") {
        Ok(folders) => {
            println!("List fodlers");
            for folder in folders.into_iter() {
                println!("- {:?}", folder);
            }
        }
        Err(e) => println!("Error listing folders: {}", e),
    };

    match imap_socket.select("INBOX") {
        Ok(mailbox) => {
            println!("{}", mailbox);
        }
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    match imap_socket.fetch("2", "body[text]") {
        Ok(messages) => {
            for message in messages.iter() {
                print!("{:?}", message);
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };

    imap_socket.logout()
}

struct MailClient;
impl MailClient {

    fn test_connect(&self, username: String, password: String) -> String 
    {
        match self.test_connect_internal() {
            Ok(_) => String::from("Success"),
            Err(e) => format!("ERROR: {}", e),
        }
    }

    fn test_connect_internal(&self)
    -> imap::error::Result<()>
    {
        let server = rail_imap::Server::load_conf("settings.json")
            .map_err(|e| imap::error::Error::Io(
                std::io::Error::new(std::io::ErrorKind::Other, e))
            )?;

        let mut connection = server.connect()?;

        println!("FOLDERS:");
        for folder in connection.list_folders()?.iter() {
            println!("\n- {:?}", folder);
            let f = rail_imap::Folder::new(&mut connection, folder);    
        }

        Ok(())
    }
}

Q_OBJECT! { MailClient:
    slot fn test_connect(String, String);
}

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("MailClient", MailClient);
    engine.load_local_file("qml/main_ui.qml");

    engine.exec();
}