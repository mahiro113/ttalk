use clap::Parser;
use ttalk::Cli;

fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}, chat: {:?}", &args.key, &args.text,args.chat)
}
