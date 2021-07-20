use clap::Clap;

#[derive(Clap)]
pub struct Opt {
    #[clap(long)]
    pub p: u128,
    #[clap(long)]
    pub q: u128,
    #[clap(long)]
    pub plaintext: u128,
}
