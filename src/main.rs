use clap::Parser;
use rconcrete::{
    cli::{Cli, Command},
    commands::{self, server::ServerCommand},
    configfile::ConfigurationFile,
    error::Error,
};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    match rt.block_on(rconcrete_main()) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn rconcrete_main() -> Result<(), Error> {
    let cli = Cli::parse();

    let config_file = ConfigurationFile::build_default()?;

    match cli.command {
        Command::Connect(connect_args) => {
            commands::connect::run(connect_args, config_file.read()?).await?;
        }
        Command::Server { command } => match command {
            ServerCommand::Add(add_args) => commands::server::run_add(add_args, config_file)?,

            ServerCommand::Remove(remove_args) => {
                commands::server::run_remove(remove_args, config_file)?
            }
            ServerCommand::List(list_args) => commands::server::run_list(list_args, config_file)?,
        },
    }
    Ok(())
}
