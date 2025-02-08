use clap::Parser;

/// Modify source files by embedding metadata needed for trait tags in item listings.
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = ".")]
    /// Workspace root directory
    root_dir: String,

    #[arg(num_args(1..))]
    /// Names of packages to modify in
    packages: Vec<String>,
}

fn main() {
    let args = Args::parse();
    embed_trait_info::run(&args.root_dir, args.packages);
}
