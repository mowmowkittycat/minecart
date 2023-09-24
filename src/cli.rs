mod actions;
mod data;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.is_empty() {
        actions::help::exec(args);
        std::process::exit(0);
    }
    let action = args.remove(0);
    match action.as_str() {
        "search" => actions::search::exec(args).await,
        "help" => {
            actions::help::exec(args);
        }
        "h" => {
            actions::help::exec(args);
        }
        _ => {
            actions::help::exec(args);
        }
    };
}
