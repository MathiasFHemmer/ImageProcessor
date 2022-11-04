use clap:: {
    Args,
    Subcommand
};

#[derive(Debug, Args)]
pub struct RepositoryArgs{
    #[clap(subcommand)]
    pub commands: RepositoryCommands
}

#[derive(Debug, Subcommand)]
pub enum RepositoryCommands{
    List,
    Exclude(ExcludeArgs),
    Add,
    Checkout(CheckoutArgs),
    Save(SaveArgs)
}

#[derive(Debug, Args)]
pub struct CheckoutArgs{
    pub index: usize
}

#[derive(Debug, Args)]
pub struct ExcludeArgs{
    pub index: Option<usize>
}

#[derive(Debug, Args)]
pub struct SaveArgs{
    pub index: Option<String>
}