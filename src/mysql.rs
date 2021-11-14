use mysql::{Conn, OptsBuilder};

#[derive(Deserialize)]
pub struct MySQLCredentials {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

pub async fn open_connection(credentials: Box<MySQLCredentials>) -> Conn {
    let opts = OptsBuilder::new()
        .db_name(Some(credentials.database))
        .user(Some(credentials.username))
        .pass(Some(credentials.password))
        .tcp_port(credentials.port);

    Conn::new(opts)?
}