use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RpcRequest<T> {
    jsonrpc: String,
    id: u64,
    method: String,
    params: T,
}

#[derive(Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: T,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct PushFileParams {
    filename: String,
    content: String,
    server: String,
}

type GetDefinitionFileParams = ();

impl RpcRequest<PushFileParams> {
    pub fn push_file(
        id: u64,
        server: &str,
        filename: &str,
        content: String,
    ) -> RpcRequest<PushFileParams> {
        RpcRequest::<_> {
            jsonrpc: "2.0".to_owned(),
            id,
            method: "pushFile".to_owned(),
            params: PushFileParams {
                filename: filename.to_owned(),
                content,
                server: server.to_owned(),
            },
        }
    }
}

impl RpcRequest<GetDefinitionFileParams> {
    pub fn get_definition_file(id: u64) -> RpcRequest<GetDefinitionFileParams> {
        RpcRequest::<_> {
            jsonrpc: "2.0".to_owned(),
            id,
            method: "getDefinitionFile".to_owned(),
            params: (),
        }
    }
}
