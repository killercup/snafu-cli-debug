use snafu::{OptionExt, ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;

fn main() -> Result<(), Error> {
    let path = &std::env::args().nth(1).context(WrongUsage)?;
    let _data = std::fs::read_to_string(&path).context(CantReadFile { path })?;

    Ok(())
}

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    #[snafu(display("Invalid usage"))]
    WrongUsage,
    #[snafu(display("Can't read file `{}`", path.display()))]
    CantReadFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
}
