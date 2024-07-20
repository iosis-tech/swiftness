use swiftness_commitment::table;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Table Error")]
    Table(#[from] table::decommit::Error),
}
