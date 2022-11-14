use minigrep::Config;
use std::env;
use std::process;

//args() will fail if the arguments is not unicode
// use args_os() instead
// first value is the binary that runs

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("App Erro exited: {e}");
        process::exit(1);
    }
}
