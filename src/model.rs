use rpc_router::RpcResource;
use std::sync::Arc;

#[derive(Default, Clone, RpcResource)]
pub struct ModelManager {
	db_client: Arc<Vec<String>>,
}
