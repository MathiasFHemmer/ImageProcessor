use clap:: {
    Args,
    Subcommand
};

#[derive(Debug, Args)]
pub struct NoiseArgs{
    #[clap(subcommand)]
    pub commands: NoiseCommands
}

#[derive(Debug, Subcommand)]
pub enum NoiseCommands{
    SaltAndPepper(SaltAndPepperArgs)
}

#[derive(Debug, Args)]
pub struct SaltAndPepperArgs{
    pub frequency: u32
}