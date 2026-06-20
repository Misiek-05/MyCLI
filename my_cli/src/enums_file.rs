#[derive(clap::Subcommand)]
pub enum Commands {

    /// Welcome command
    Welcome { 
        /// Number of times to say "HI". 
        #[arg(short, long)]
        count: Option<u8>,
    }, 
    /// Create new file
    Touch {
        /// name of the file (and path)
        #[arg(short, long)]
        name: String,
    }, 
    /// Print file
    Cat {
        /// file to print into command line
        #[arg(short, long)]
        name: String,

        /// amount of words to print from file
        #[arg(short = 'b', long = "number")]
        amount: Option<i32>
    },
    /// Curent working directory
    Pwd,
    Ls,
    /// Copy
    Cp {
        /// source file
        #[arg(short = 's', long = "source")]
        source_name: String,

        /// destination file
        #[arg(short = 'd', long = "dest")]
        dest_name: String,
    },
    /// move/rename file (Can both)
    Mv {
        /// source file
        #[arg(short = 's', long = "source")]
        source_name: String,

        /// destination file
        #[arg(short = 'd', long = "dest")]
        dest_name: String,
    },
    /// create directory
    Mkdir {
        /// name of the new directory (and path)
        #[arg(short, long)]
        name: String,
    },
}