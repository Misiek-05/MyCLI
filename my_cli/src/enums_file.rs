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
        /// name of the files (and path)
        #[arg(short, long)]
        name: String,

        /// extension of the file
        #[arg(short, long)]
        extension: String
    }, 
    // Print file
    Cat {
        /// file to print into command line
        #[arg(short, long)]
        name: String,

        /// amount of words to print from file
        #[arg(short = 'a', long = "amount")]
        amount: Option<i32>
    },
    // Curent working directory
    Pwd,
    Ls 
}