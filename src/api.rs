use leptos::{server, ServerFnError};

use crate::model::conversation::Conversation;

#[server(name = Converse, prefix = "/api", endpoint = "converse")]
pub async fn converse (prompt: Conversation) -> Result<String, ServerFnError> {
}
