use async_openai::types::{
	ChatCompletionToolChoiceOption, CreateChatCompletionRequest,
};
use rpc_router::{router_builder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use xp_ai_function::oa_client::new_oa_client;
use xp_ai_function::{chat, gpts};

#[allow(unused)] // Will be passthrough API
#[derive(Debug, Deserialize, RpcParams)]
struct GetWeatherParams {
	location: String,
	country: String,
	unit: String,
}

#[derive(Serialize)]
struct Weather {
	temperature: f64,
	unit: String,
	humidity_rh: f32,
}

async fn get_weather(params: GetWeatherParams) -> Result<Weather, String> {
	Ok(Weather {
		temperature: 30.,
		unit: params.unit,
		humidity_rh: 0.3,
	})
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Init AI Client
	let oa_client = new_oa_client()?;
	let chat_client = oa_client.chat();
	let model = gpts::MODEL;

	// -- User question
	let question = "What is the weather in the California's best city and Paris?";

	// -- Build messages
	let messages = vec![chat::user_msg(question)?];

	// -- Build tools
	let tool_weather = chat::tool_fn(
		"get_weather",
		"get the weather for a city",
		json!({
			"type": "object",
			"properties": {
				"location": {
					"type": "string",
					"description": "The city and state, e.g. San Francisco, CA"
				},
				"country": {
					"type": "string",
					"description": "The full country name of the city"
				},
				"unit": {
					"type": "string", "enum": ["celsius", "fahrenheit"],
					"description": "Unit respecting the country of the city"
				},
			},
			"required": ["location", "country", "unit"],
		}),
	)?;
	let tools = Some(vec![tool_weather]);

	// -- Init rpc_router
	let rpc_router = router_builder![get_weather].build();

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
		println!("\nResponse early (no tools):\n\n{response_content}");
		return Ok(());
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

	println!("\nFinal answer:\n\n{content}");

	Ok(())
}
