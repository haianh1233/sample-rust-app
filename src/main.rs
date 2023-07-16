use clap::Parser;

mod ping;

#[derive(Parser)]
#[clap(name = "MyApp")]
enum Cli {
    #[clap(name = "ping", about = "Ping a URL")]
    Ping {
        #[clap(help = "URL to ping")]
        url: String,
    },
    #[clap(name = "main", about = "Process a file")]
    Main {
        #[clap(help = "Path to file")]
        path: std::path::PathBuf,
    },
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    match args {
        Cli::Ping { url } => {
            let _ = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(ping::run_ping(&url));
        }
        Cli:: Main { path } => {
            let path = path.to_str().unwrap_or_else(|| {
                panic!("Invalid path");
            });
            let content = std::fs::read_to_string(path)
                .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
            println!("file content: {}", content);

        }
    }

    Ok(())
}