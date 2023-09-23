use clap::{arg, command, Arg, Command, Subcommand};

fn main() {
    let matches = Command::new("kvs")
        .version("0.1.0")
        .about("Does awesome things")
        .subcommand(
            Command::new("get")
                .about("Gets a value")
                .arg(Arg::new("KEY").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Removes a value")
                .arg(Arg::new("KEY").required(true)),
        )
        .subcommand(
            Command::new("set")
                .about("Sets a value")
                .arg(Arg::new("KEY").required(true))
                .arg(Arg::new("VALUE").required(true)),
        )
        .get_matches();

    if matches.subcommand_name().is_none() {
        std::process::exit(1);
    }

    match matches.subcommand() {
        Some(("get", sub_m)) if sub_m.get_one::<String>("KEY").is_some() => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Some(("rm", sub_m)) if sub_m.get_one::<String>("KEY").is_some() => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Some(("set", sub_m))
            if sub_m.get_one::<String>("KEY").is_some()
                && sub_m.get_one::<String>("VALUE").is_some() =>
        {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        _ => {}
    }
}
