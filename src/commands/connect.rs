use std::io::{stdin, stdout, Write};

use clap::Parser;
use rcon::Builder;

use crate::error::Error;
use crate::{commands::execute_command, configfile::Configuration};

#[derive(Parser)]
pub struct ConnectArgs {
    server: String,
}

pub async fn run(args: ConnectArgs, config: Configuration) -> Result<(), Error> {
    let target = config
        .known_servers
        .iter()
        .find(|server| server.name == args.server)
        .ok_or(Error::NoServersExisted)?;

    let address = &target.address;

    let password = &target.password;

    let mut connection = target
        .enable_quirks(Builder::new())
        .connect(address, password)
        .await
        .map_err(|x| match x {
            rcon::Error::Auth => Error::AuthenticationError,
            rcon::Error::CommandTooLong => Error::CommandTooLong,
            rcon::Error::Io(x) => Error::ConnectionError(x),
        })?;

    println!("use CTRL-C to exit");
    let mut stdout = stdout();
    let stdin = stdin();
    let mut buf = String::new();

    loop {
        print!("{} > ", target.name);
        stdout.flush().map_err(Error::IoError)?;
        stdin.read_line(&mut buf).map_err(Error::IoError)?;

        println!("{}", execute_command(&mut connection, buf.trim()).await?);
        buf.clear();
    }
}
