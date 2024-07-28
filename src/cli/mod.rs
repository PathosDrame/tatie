use clap::Parser;
use connect::ConnectOpts;
use describe::DescribeOpts;
use head::HeadOpts;
use sql::SqlOpts;

mod connect;
mod describe;
mod head;
mod list;
mod sql;

pub use self::{connect::connect, describe::describe, head::head, list::list, sql::sql};

type ReplResult = Result<Option<String>, reedline_repl_rs::Error>;

#[derive(Debug, Parser)]
pub enum ReplCommand {
    #[command(
        name = "connect",
        about = "Connect to a dataset and register it to TaoTie"
    )]
    Connect(ConnectOpts),
    #[command(name = "list", about = "List all registered datasets")]
    List,
    #[command(name = "describe", about = "Describe a dataset")]
    Describe(DescribeOpts),
    #[command(about = "Show the first few rows of a dataset")]
    Head(HeadOpts),
    #[command(about = "Run a SQL query on a dataset")]
    Sql(SqlOpts),
}
