use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

/// VDS - A Git-like Versioned Data Store
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new VDS repository
    Init,
    /// Add files to the staging area
    Add { files: Vec<String> },
    /// Commit staged changes
    Commit { 
        #[arg(short, long)]
        message: String 
    },
    /// Show commit history
    Log,
    /// Switch to a different commit or branch
    Checkout { target: String },
    /// Create or switch branches
    Branch { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            if let Err(e) = init_repository() {
                eprintln!("Error initializing repository: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Add { files } => {
            println!("Adding files: {:?}", files);
            // TODO: Implement add functionality
        }
        Commands::Commit { message } => {
            println!("Committing with message: {}", message);
            // TODO: Implement commit functionality
        }
        Commands::Log => {
            println!("Showing commit log");
            // TODO: Implement log functionality
        }
        Commands::Checkout { target } => {
            println!("Checking out: {}", target);
            // TODO: Implement checkout functionality
        }
        Commands::Branch { name } => {
            match name {
                Some(branch_name) => println!("Creating/switching to branch: {}", branch_name),
                None => println!("Listing branches"),
            }
            // TODO: Implement branch functionality
        }
    }
}

/// Initialize a new VDS repository
fn init_repository() -> Result<(), Box<dyn std::error::Error>> {
    let vds_dir = ".vds";
    
    // Check if repository already exists
    if Path::new(vds_dir).exists() {
        return Err("VDS repository already exists in this directory".into());
    }
    
    // Create the main .vds directory
    fs::create_dir(vds_dir)?;
    
    // Create subdirectories
    fs::create_dir_all(format!("{}/objects", vds_dir))?;
    fs::create_dir_all(format!("{}/refs/heads", vds_dir))?;
    
    // Create HEAD file pointing to main branch
    fs::write(format!("{}/HEAD", vds_dir), "ref: refs/heads/main\n")?;
    
    // Create empty index file
    fs::write(format!("{}/index", vds_dir), "")?;
    
    println!("Initialized empty VDS repository in .vds/");
    
    Ok(())
}
