use clap:: {
    Args,
    Subcommand
};

#[derive(Debug, Args)]
pub struct TransformArgs{
    #[clap(subcommand)]
    pub commands: TransformCommands
}

#[derive(Debug, Subcommand)]
pub enum TransformCommands{
    Fourier
}

#[derive(Debug, Args)]
pub struct SaltAndPepperArgs{
    pub frequency: u32
}