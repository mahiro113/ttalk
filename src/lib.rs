use clap::Parser;

#[derive(Debug,Parser)]
pub struct Cli {
    #[arg(short,long,required=true,help="GeminiのAPIKEYをわたしてください")]
    pub key: String,

    #[arg(short,long,required=true,help="Geminiと会話する内容を入力してください")]
    pub text: String,

    #[arg(short,long,action=clap::ArgAction::SetFalse,help="対話形式で話したい場合は引数に追加してください")]
    pub chat: bool
} 