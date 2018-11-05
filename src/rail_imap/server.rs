use rail_imap::{
    Connection,
    TLSConnection,
    InsecureConnection,
};
use imap::error::Result as ImapResult;
use imap::client as imap_client;

pub struct Server {
    domain: String,
    port: u16,
    ssl: bool,
    //TODO https://docs.rs/secret-service/0.4.0/secret_service/
    username: String,
    password: String,
}

impl Server {
    pub fn new(
        domain: String,
        port: u16,
        ssl: bool,
        username: String,
        password: String,
    ) -> Self {
        Self {
            domain,
            port,
            ssl,
            username,
            password,
        }
    }

    pub fn load_conf(file: &str)
    -> Result<Self, config::ConfigError>
    {
        use config::{File, Config};

        let mut settings = Config::default();
        settings.merge(File::with_name(file))?;

        let domain:   String = settings.get("domain")?;
        let port:     u16    = settings.get("port")?;
        let ssl:      bool   = settings.get("ssl")?;
        let username: String = settings.get("username")?;
        let password: String = settings.get("password")?;

        Ok(Self {
            domain,
            port,
            ssl,
            username,
            password,
        })
    }

    pub fn connect(&self) -> ImapResult<Box<Connection>>
    {
        if self.ssl {
            self.connect_secure()
        } else {
            self.connect_insecure()
        }
    }

    fn connect_insecure(&self) -> ImapResult<Box<Connection>>
    {
        println!("WARNING! You are connecting not securely to the server");
        let socket_addr = (self.domain.as_ref(), self.port);
  
        let client = imap_client::connect(socket_addr)?;
        let imap_socket = client.login(&self.username, &self.password)
            .map_err(|(error, _client)| error)?;
        Ok(Box::new(InsecureConnection::new(imap_socket)) as Box<Connection>)
    }

    fn connect_secure(&self) -> ImapResult<Box<Connection>> {
        let socket_addr = (self.domain.as_ref(), self.port);
  
        let ssl_connector = native_tls::TlsConnector::builder().build()?;
        let client = imap_client::secure_connect(socket_addr, &self.domain, &ssl_connector)?;
        let imap_socket = client.login(&self.username, &self.password)
            .map_err(|(error, _client)| error)?;
        Ok(Box::new(TLSConnection::new(imap_socket)) as Box<Connection>)
    }
}