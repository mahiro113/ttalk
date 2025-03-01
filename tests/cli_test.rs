use ttalk::Cli;
use clap::Parser;

#[test]
fn test(){
    let arg = Cli::parse();
    println!("{:?}",arg)
}
