pub use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "aria2xspf")]
pub struct Opts {
    /// Emit html instead of xspf
    #[structopt(short = "h", long = "html")]
    pub html: bool,

    /// Aria2 formatted input
    #[structopt(name = "IN", parse(from_os_str), default_value = "VIDs")]
    pub input: PathBuf,

    /// Output
    #[structopt(name = "OUT", parse(from_os_str), default_value = "VIDs.xspf")]
    pub output: PathBuf,
}
