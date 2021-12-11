use diesel::Queryable;

#[derive(Clone, Debug, Queryable)]
pub struct TransactionTable {
    pub id: i64,
    pub tx_hash: Vec<u8>,
    pub tx_index: u32,
    pub input_count: u32,
    pub output_count: u32,
    pub block_number: u64,
    pub block_hash: Vec<u8>,
    pub tx_timestamp: u64,
    pub version: u16,
    pub cell_deps: Vec<u8>,
    pub header_deps: Vec<u8>,
    pub witnesses: Vec<u8>,
}

#[derive(Clone, Debug, Queryable)]
pub struct CellTable {
    pub id: i64,
    pub tx_hash: Vec<u8>,
    pub output_index: u32,
    pub tx_index: u32,
    pub block_number: u64,
    pub block_hash: Vec<u8>,
    pub epoch_number: u32,
    pub epoch_index: u32,
    pub epoch_length: u32,
    pub capacity: u64,
    pub lock_hash: Vec<u8>,
    pub lock_code_hash: Vec<u8>,
    pub lock_args: Vec<u8>,
    pub lock_script_type: u8,
    pub type_hash: Vec<u8>,
    pub type_code_hash: Vec<u8>,
    pub type_args: Vec<u8>,
    pub type_script_type: u8,
    pub data: Vec<u8>,
    pub consumed_block_number: Option<u64>,
    pub consumed_block_hash: Vec<u8>,
    pub consumed_tx_hash: Vec<u8>,
    pub consumed_tx_index: Option<u32>,
    pub input_index: Option<u32>,
    pub since: Vec<u8>,
}

#[derive(Clone, Debug, Queryable)]
pub struct IndexerCellTable {
    pub id: i64,
    pub block_number: u64,
    pub io_type: u8,
    pub io_index: u32,
    pub tx_hash: Vec<u8>,
    pub tx_index: u32,
    pub lock_hash: Vec<u8>,
    pub lock_code_hash: Vec<u8>,
    pub lock_args: Vec<u8>,
    pub lock_script_type: u8,
    pub type_hash: Vec<u8>,
    pub type_code_hash: Vec<u8>,
    pub type_args: Vec<u8>,
    pub type_script_type: u8,
}


