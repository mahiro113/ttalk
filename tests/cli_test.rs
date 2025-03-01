extern crate ttalk;
use clap::Parser;

#[test]
fn test(){
    let arg = ttalk::Cli::parse();
    println!("{:?}",arg)
}
