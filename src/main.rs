use clap::Parser;


use lf::{output::{print_files_in_table, list_files_long}, file_operations::Files};
use lf::file_operations::get_file_names_in_dir;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool, // show all files
    #[arg(short, long)]
    long: bool,


    #[clap(default_value = ".")]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cli_args = Cli::parse();

    if !cli_args.path.ends_with('/') {
        cli_args.path.push('/');
    }

    let mut files = Files {
        dir: cli_args.path,
        files: Vec::new(),
    };
    
    files.files = get_file_names_in_dir(&files.dir, cli_args.all)?;

    if cli_args.long {
        list_files_long(&files);

    } else {
        print_files_in_table(&mut files);
    }

    Ok(())
}
