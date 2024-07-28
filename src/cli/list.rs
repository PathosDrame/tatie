use super::{ReplCommand, ReplResult};
use crate::ReplContext;
use clap::ArgMatches;

pub fn list(_args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    ctx.send(ReplCommand::List);

    Ok(None)
}
