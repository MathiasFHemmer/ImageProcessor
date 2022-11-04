use clap:: {
    Parser,
    Subcommand
};

use super::repository;
use super::image::image;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Commands{
    #[clap(subcommand)]
    pub operation: OperationType
}

#[derive(Debug, Subcommand)]
pub enum OperationType{
    /// Apply a gaussian noise to the image
    Repository(repository::RepositoryArgs),
    Image(image::ImageArgs)
}

