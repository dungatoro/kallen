use clap::{ Args, Parser, Subcommand };

#[derive(Parser)]
#[clap(author, version, about)]
pub struct KallenArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Add(NewEvent),
    Del(BadEvent),
    Update(UpdatedEvent),
    Todays
}

#[derive(Debug, Args)]
pub struct NewEvent {
    #[arg(long, default_value_t = String::from(""))]
    pub date: String,

    #[arg(short,long, default_value_t = String::from(""))]
    pub time: String,

    #[arg(long)]
    pub desc: String,
}

#[derive(Debug, Args)]
pub struct BadEvent {
    #[arg(long, default_value_t = String::from(""))]
    pub date: String,

    #[arg(short,long)]
    pub idx: usize,
}

#[derive(Debug, Args)]
pub struct UpdatedEvent {
    #[arg(long, default_value_t = String::from(""))]
    pub date: String,

    #[arg(short,long, default_value_t = String::from(""))]
    pub time: String,

    #[arg(short,long)]
    pub idx: usize,

    #[arg(long)]
    pub desc: String,
}
