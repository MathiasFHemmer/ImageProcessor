use clap:: {
    Args,
    Subcommand
};

use super::{filter, noise,transform};

#[derive(Debug, Args)]
pub struct ImageArgs{
    #[clap(subcommand)]
    pub commands: ImageCommands
}

#[derive(Debug, Subcommand)]
pub enum ImageCommands{
    Add(AddArgs),
    Undo,
    Checkout(CheckoutArgs),
    Clone,
    Grayscale,
    List,
    Show,
    Filter(filter::FilterArgs),
    Noise(noise::NoiseArgs),
    Transform(transform::TransformArgs),
    Histogram,
    PSNR(PSNRArgs)
}

#[derive(Debug, Args)]
pub struct AddArgs{
    pub path: String
}

#[derive(Debug, Args)]
pub struct CheckoutArgs{
    pub id: String
}

#[derive(Debug, Args)]
pub struct ShowArgs{
}

#[derive(Debug, Args)]
pub struct PSNRArgs{
    pub path: String
}