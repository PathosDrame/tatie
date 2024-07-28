use clap::{ArgMatches, Parser};

use crate::ReplContext;

use super::{ReplCommand, ReplResult};

#[derive(Debug, Parser)]
pub struct HeadOpts {
    #[arg(short, long, help = "The name of the dataset")]
    pub name: String,

    #[arg(short, long, help = "The number of rows to show")]
    pub num_rows: Option<usize>,
}

pub fn head(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expected name")
        .to_string();

    let num_rows = args.get_one::<usize>("num_rows").copied();

    let cmd = HeadOpts::new(name, num_rows).into();

    ctx.send(cmd);

    Ok(None)
}

impl From<HeadOpts> for ReplCommand {
    fn from(opts: HeadOpts) -> Self {
        ReplCommand::Head(opts)
    }
}

impl HeadOpts {
    pub fn new(name: String, num_rows: Option<usize>) -> Self {
        Self { name, num_rows }
    }
}
