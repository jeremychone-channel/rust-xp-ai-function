use rpc_router::RpcResource;
use std::sync::Arc;

#[derive(Default, Clone, RpcResource)]
pub struct ModelManager {
	#[allow(unused)] // just for demonstration of a owned stack object
	db_client: Arc<Vec<String>>,
}
