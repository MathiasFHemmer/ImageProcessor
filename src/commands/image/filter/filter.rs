use clap:: {
    Args,
    Subcommand
};

#[derive(Debug, Args)]
pub struct FilterArgs{
    #[clap(subcommand)]
    pub commands: FilterCommands
}

#[derive(Debug, Subcommand)]
pub enum FilterCommands{
    Median(MedianArgs),
    LowPass(LowPassArgs),
    LowPassSmooth(LowPassSmoothArgs)
}

#[derive(Debug, Args)]
pub struct MedianArgs{
    pub kernel_size: f32
}

#[derive(Debug, Args)]
pub struct LowPassArgs{
    pub radius: f64
}

#[derive(Debug, Args)]
pub struct LowPassSmoothArgs{
    pub inner: f64,
    pub outer: f64
}