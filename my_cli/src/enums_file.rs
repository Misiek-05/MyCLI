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
        name: String,
    }, 
    /// Print file
    Cat {
        /// file to print into command line
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
        source_name: String,

        /// destination file
        dest_name: String,
    },
    Rm {
        /// name of the file
        name: String,
    },
    /// move/rename file (Can both)
    Mv {
        /// source file
        source_name: String,

        /// destination file
        dest_name: String,
    },
    /// create directory
    Mkdir {
        /// name of the new directory (and path)
        name: String,
    },
    /// Display lines from the back of the file
    Tail {
        /// name of the file
        name: String,

        /// amount of lines (Default = 10)
        #[arg(short, long)]
        count: Option<u8>,
    },
    /// Display lines from the begining of the file
    Head {
        /// name of the file
        name: String,

        /// amount of lines (Default = 10)
        #[arg(short, long)]
        count: Option<u8>,
    },
    Echo {
        /// Text input
        text: String,

        /// optional file to print text into
        #[arg(short, long)]
        name: Option<String>
    },
    /// Display calendar
    Cal

}