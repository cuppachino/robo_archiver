use clap::{ command, Parser };

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// A directory containing files to process (optional).
    ///
    /// If not provided, the program will search for files in the current directory.
    #[arg(short, long)]
    pub file_dir: Option<String>,

    /// If provided, the program will search for files in the directory recursively (optional).
    ///
    /// Defaults to false.
    #[arg(short, long)]
    pub recursive: bool,

    /// The call number of the periodical (optional).
    ///
    /// If not provided, the field will be interpreted as `PERIODICAL` and left blank.
    #[arg(short = 'n', long)]
    pub call_number: Option<String>,

    /// The Periodical Collection to which the periodical belongs (optional).
    ///
    /// Defaults to "Arizona Collection|Arizona Periodicals and Magazines".
    #[arg(long)]
    pub collection: Option<String>,

    /// The Contributing Institution that owns the periodical (optional).
    ///
    /// Defaults to "State of Arizona Research Library- Arizona State Library, Archives and Public Records".
    #[arg(short = 'i', long)]
    pub contributing_institution: Option<String>,

    /// The institution that digitized the periodical (optional).
    ///
    /// Defaults to "FamilySearch International".
    #[arg(long)]
    pub digitization_institution: Option<String>,

    /// The copyright statement to include per issue (optional).
    ///
    /// Defaults to "NO COPYRIGHT - UNITED STATES. [..abbreviated..] http://rightsstatements.org/vocab/NoC-US/1.0/"
    #[arg(short = 'c', long)]
    pub rights_statement: Option<String>,
}
