use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct KallenArgs {
    #[arg(short,long)]
    pub action: usize,

    #[arg(short,long, default_value_t = String::from(""))]
    pub time: String,

    #[arg(short,long, default_value_t = String::from(""))]
    pub desc: String,

}
