use rcon::Connection;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::error::Error;

pub mod connect;
pub mod server;

async fn execute_command<T: Unpin + AsyncRead + AsyncWrite>(
    conn: &mut Connection<T>,
    cmd: &str,
) -> Result<String, Error> {
    conn.cmd(cmd).await.map_err(|x| match x {
        rcon::Error::Auth => Error::AuthenticationError,
        rcon::Error::CommandTooLong => Error::CommandTooLong,
        rcon::Error::Io(e) => Error::ConnectionError(e),
    })
}
