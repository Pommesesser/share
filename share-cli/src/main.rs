mod http;
mod printer;

use std::env;

#[tokio::main]
async fn main() {
    let mut args = env::args();

    let _program = args.next();

    let command = match args.next() {
        Some(command) => command,
        None => {
            println!("usage: share <command>");
            return;
        }
    };

    match command.as_str() {
        "upload" => {
            let path = match args.next() {
                Some(path) => path,
                None => {
                    println!("usage: share upload <path>");
                    return;
                }
            };

            let id = http::upload(&path).await;
            printer::print_link(http::SERVER, &id);
        }

        "ls" => {
            let file_entries = http::list().await;
            printer::print_file_entries(&file_entries);
        }

        "get" => {
            let id = match args.next() {
                Some(id) => id,
                None => {
                    println!("usage: share get <id>");
                    return;
                }
            };

            let filename = http::get(&id).await;
            println!("downloaded {filename}");
        }

        "rm" => {
            let id = match args.next() {
                Some(id) => id,
                None => {
                    println!("usage: share rm <id>");
                    return;
                }
            };

            let filename = http::remove(&id).await;
            println!("downloaded {filename}");
        }

        _ => {
            println!("unknown command: {command}");
        }
    }
}
