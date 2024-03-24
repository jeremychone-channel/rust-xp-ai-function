use crate::{tools, Result};
use async_openai::types::{
	ChatChoice, ChatCompletionMessageToolCall,
	ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
	ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs,
	ChatCompletionTool, ChatCompletionToolArgs, CreateChatCompletionResponse,
	FunctionObject,
};
use schemars::JsonSchema;
use serde_json::Value;
use std::fmt::Display;

pub fn user_msg(content: impl Into<String>) -> Result<ChatCompletionRequestMessage> {
	let msg = ChatCompletionRequestUserMessageArgs::default()
		.content(content.into())
		.build()?;
	Ok(msg.into())
}

pub fn tool_response_msg(
	tool_call_id: String,
	content: impl Display,
) -> Result<ChatCompletionRequestMessage> {
	let msg = ChatCompletionRequestToolMessageArgs::default()
		.tool_call_id(tool_call_id)
		.content(content.to_string())
		.build()?;
	Ok(msg.into())
}

pub fn tool_calls_msg(
	tool_calls: Vec<ChatCompletionMessageToolCall>,
) -> Result<ChatCompletionRequestMessage> {
	let msg = ChatCompletionRequestAssistantMessageArgs::default()
		.tool_calls(tool_calls)
		.build()?;

	Ok(msg.into())
}

pub fn tool_fn_from_type<T: JsonSchema>() -> Result<ChatCompletionTool> {
	let spec = tools::tool_spec::<T>()?;
	tool_fn(spec.fn_name, spec.fn_description, spec.params)
}

pub fn tool_fn(
	name: impl Into<String>,
	description: impl Into<String>,
	parameters: Value,
) -> Result<ChatCompletionTool> {
	let tool = ChatCompletionToolArgs::default()
		.function(FunctionObject {
			name: name.into(),
			description: Some(description.into()),
			parameters: Some(parameters),
		})
		.build()?;
	Ok(tool)
}

pub fn first_choice(
	chat_response: CreateChatCompletionResponse,
) -> Result<ChatChoice> {
	let first_choice = chat_response
		.choices
		.into_iter()
		.next()
		.ok_or("No first choice?")?;
	Ok(first_choice)
}
