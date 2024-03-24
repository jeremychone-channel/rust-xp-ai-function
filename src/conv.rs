use crate::oa_client::OaClient;
use crate::tools::AiTools;
use crate::{chat, gpts, Result};
use async_openai::types::{
	ChatCompletionToolChoiceOption, CreateChatCompletionRequest,
};
use serde_json::Value;

pub async fn send_user_msg(
	oa_client: OaClient,
	ai_tools: AiTools,
	question: &str,
) -> Result<String> {
	let chat_client = oa_client.chat();
	let model = gpts::MODEL;

	// -- Build messages
	let messages = vec![chat::user_msg(question)?];

	// -- Extract tools and rpc_router
	let rpc_router = ai_tools.router();
	let tools = Some(ai_tools.chat_tools_clone());

	// -- Exec Chat Request
	let msg_req = CreateChatCompletionRequest {
		model: model.to_string(),
		messages: messages.clone(),
		tools: tools.clone(),
		tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
		..Default::default()
	};
	let chat_response = chat_client.create(msg_req).await?;
	let first_choice = chat::first_choice(chat_response)?;

	// -- If message.content, end early
	if let Some(response_content) = first_choice.message.content {
		return Ok(response_content);
	}

	// -- Otherwise, get/call tools/rpc calls and capture the Tool Responses
	struct ToolResponse {
		tool_call_id: String,
		/// Response value of the rpc_router call
		response: Value,
	}
	let mut tool_responses: Vec<ToolResponse> = Vec::new();

	// For each tool_call, rpc_router call
	let tool_calls = first_choice.message.tool_calls;
	for tool_call in tool_calls.iter().flatten() {
		let tool_call_id = tool_call.id.clone();
		let fn_name = tool_call.function.name.clone();
		let params: Value = serde_json::from_str(&tool_call.function.arguments)?;

		// Execute with rpc_router
		let call_result = rpc_router.call_route(None, fn_name, Some(params)).await?;
		let response = call_result.value;

		// Add it to the tool_responses
		tool_responses.push(ToolResponse {
			tool_call_id,
			response,
		});
	}

	// -- Make messages mutable for follow-up
	let mut messages = messages;

	// -- Append the tool calls (send from AI Model)
	if let Some(tool_calls) = tool_calls {
		messages.push(chat::tool_calls_msg(tool_calls)?);
	}

	// -- Append the Tool Responses (computed by this code)
	for ToolResponse {
		tool_call_id,
		response,
	} in tool_responses
	{
		messages.push(chat::tool_response_msg(tool_call_id, response)?);
	}

	// -- Exec second request with tool responses
	let msg_req = CreateChatCompletionRequest {
		model: model.to_string(),
		messages,
		tools,
		tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
		..Default::default()
	};
	let chat_response = chat_client.create(msg_req).await?;
	let first_choice = chat::first_choice(chat_response)?;

	// -- Get the final response
	let content = first_choice.message.content.ok_or("No final content?")?;

	Ok(content)
}
