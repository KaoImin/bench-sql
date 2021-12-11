use jsonrpsee_proc_macros::rpc;
use jsonrpsee_http_server::types::Error;
use ckb_types::H256;

#[rpc(server)]
pub trait Rpc {
	#[method(name = "query_transactions")]
	async fn query_transactions(&self, lock_hash: H256) -> Result<(), Error>;
}
