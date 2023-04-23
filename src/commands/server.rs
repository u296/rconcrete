use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use crate::configfile::ConfigurationFile;
use crate::error::Error;
use crate::Server;

static IP_NUM_REGEX: &str = "25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?";

lazy_static! {
    static ref PORT_REGEX: Regex = Regex::new(":[0-9]+$").unwrap();
    static ref IPV4_ADDRESS_REGEX: Regex = Regex::new(&format!(
        r"^({IP_NUM_REGEX})\.({IP_NUM_REGEX})\.({IP_NUM_REGEX})\.({IP_NUM_REGEX}):[0-9]+$"
    ))
    .unwrap();
    static ref HOSTNAME_REGEX: Regex =
        Regex::new(r"^[[0-9][A-z]]+((\.|-|_)[[0-9][A-z]]+)*:[0-9]+$").unwrap();
}

#[derive(Parser)]
pub enum ServerCommand {
    Add(AddArgs),
    Remove(RemoveArgs),
    List(ListArgs),
}

#[derive(Parser)]
pub struct AddArgs {
    name: String,
    #[arg(value_parser =  valid_socket_address)]
    address: String,
    password: String,
    quirks: Vec<crate::Quirk>,
}

fn valid_socket_address(s: &str) -> Result<String, String> {
    let port_number = &PORT_REGEX
        .find(s)
        .ok_or("No valid port specified".to_owned())?
        .as_str()[1..];

    match port_number.parse::<u16>() {
        Ok(1..=std::u16::MAX) => (),
        _ => return Err("No valid port specified".to_owned()),
    }

    if !(IPV4_ADDRESS_REGEX.is_match(s) || HOSTNAME_REGEX.is_match(s)) {
        return Err("Invalid address".to_owned());
    }

    Ok(s.into())
}

pub fn run_add(args: AddArgs, config_file: ConfigurationFile) -> Result<(), Error> {
    let mut config = config_file.read()?;

    for known_server in config.known_servers.iter() {
        if known_server.name == args.name {
            return Err(Error::ServerAlreadyExists);
        }
    }

    let server = Server {
        name: args.name,
        address: args.address,
        password: args.password,
        quirks: args.quirks,
    };

    config.known_servers.push(server);

    config_file.write(&config)
}

#[derive(Parser)]
pub struct RemoveArgs {
    #[arg(num_args = 1..)]
    names: Vec<String>,
}

pub fn run_remove(args: RemoveArgs, config_file: ConfigurationFile) -> Result<(), Error> {
    let mut config = config_file.read()?;

    let mut found = false;

    config.known_servers.retain(|known_server| {
        if !args.names.contains(&known_server.name) {
            true
        } else {
            found = true;
            false
        }
    });

    if !found {
        return Err(Error::NoServersExisted);
    }

    config_file.write(&config)
}

#[derive(Parser)]
pub struct ListArgs {
    #[arg(long)]
    show_passwords: bool,
}

pub fn run_list(args: ListArgs, config_file: ConfigurationFile) -> Result<(), Error> {
    let mut first_server = true;
    let config = config_file.read()?;
    config.known_servers.iter().for_each(|known_server| {
        if !first_server {
            println!();
        }
        first_server = false;

        println!("name: \"{}\"", known_server.name);
        println!("address: \"{}\"", known_server.address);
        print!("quirks: ");
        {
            let mut first = true;
            for quirk in known_server.quirks.iter() {
                if !first {
                    print!(", ");
                }
                first = false;
                print!("{}", quirk);
            }
            println!();
        }
        if args.show_passwords {
            println!("password: \"{}\"", known_server.password);
        }
    });
    Ok(())
}

#[cfg(test)]
mod validation_tests {
    use crate::commands::server::{HOSTNAME_REGEX, IPV4_ADDRESS_REGEX, PORT_REGEX};

    #[test]
    fn test_port_regex() {
        let success_cases = vec![
            "rt.picrtejkr234908g2<2<42+34:23403",
            "https://google.com:50234",
            "192.168.0.100:22",
        ];
        let fail_cases = vec!["helloworld:-4", "1.1.1.1:", "bruh", "hello.com:no"];

        for test in success_cases {
            assert!(PORT_REGEX.is_match(test));
        }

        for test in fail_cases {
            assert!(!PORT_REGEX.is_match(test))
        }
    }

    #[test]
    fn ipv4_address_regex_test() {
        let success_cases = vec![
            "192.168.0.1:1",
            "255.255.105.9:50",
            "127.0.0.1:25575",
            "8.8.8.8:2",
        ];

        let fail_cases = vec![
            "256.500.999.888:3",
            "172.215.186.23/24",
            "bur:43421",
            "https://youtube.com:3",
            "192.167.0.999:5",
            "165.91.95.0:35  ",
        ];

        for test in success_cases {
            assert!(IPV4_ADDRESS_REGEX.is_match(test));
        }

        for test in fail_cases {
            assert!(!IPV4_ADDRESS_REGEX.is_match(test))
        }
    }

    #[test]
    fn hostname_regex_test() {
        let success_cases = vec![
            "server:656",
            "my-website.with_underscore.tld:111",
            "hellothere1234:1",
        ];
        let failure_cases = vec!["illegal/character:143", "noport", ".atbeginning:34"];

        for test in success_cases {
            assert!(HOSTNAME_REGEX.is_match(test));
        }

        for test in failure_cases {
            assert!(!HOSTNAME_REGEX.is_match(test))
        }
    }

    #[test]
    fn validate_socket_address() {
        let success_cases = vec![
            "server:25575",
            "192.168.1.75:9",
            "100.100.50.50:50",
            "my-domain.tld:40",
            "192.167.0.999:5", // this is technically a valid HOSTNAME, but not ipv4 address
        ];
        let failure_cases = vec![
            "no-port.com:",
            "no-colon234",
            "negativeport:-324",
            "comma,in,text:1",
            "port-too-big:99999",
            "port-zero:0",
        ];

        for test in success_cases {
            assert!(super::valid_socket_address(test).is_ok());
        }

        for test in failure_cases {
            println!("case: {}", test);
            assert!(super::valid_socket_address(test).is_err());
        }
    }
}
