use clap::{crate_authors, crate_description, crate_name, crate_version, Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(author = crate_authors!(), name = crate_name!(), version = crate_version!(), about = crate_description!(), long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    let cli = Args::parse();
    let mut store = KvStore::new();

    match cli.command {
        Some(Commands::Set { key, value }) => {
            let value = store.set(key, value);
            match value {
                _ => {
                    eprintln!("unimplemented");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Get { key }) => {
            let value = store.get(key);
            match value {
                Some(value) => println!("{}", value),
                None => {
                    eprintln!("unimplemented");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Rm { key }) => {
            let value = store.remove(key);
            match value {
                _ => {
                    eprintln!("unimplemented");
                    std::process::exit(1);
                }
            }
        }
        None => std::process::exit(1),
    }
}
