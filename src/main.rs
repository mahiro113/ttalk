use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    #[arg(short,long,help="GeminiのAPIKEYをわたしてください")]
    key: String,

    #[arg(short,long,help="Geminiと会話する内容を入力してください")]
    text: String,

    #[arg(short,long,action=clap::ArgAction::SetFalse,help="対話形式で話したい場合は引数に追加してください")]
    chat: bool
} 

fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}, caht: {:?}", &args.key, &args.text,args.chat)
}
