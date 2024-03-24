use rpc_router::{router_builder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::json;
use xp_ai_function::oa_client::new_oa_client;
use xp_ai_function::tools::AiTools;
use xp_ai_function::{chat, conv};

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

	// -- User question
	let question = "What is the weather in the California's best city and Paris?";

	// -- Build tools
	let rpc_router = router_builder![get_weather].build();
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

	let ai_tools = AiTools::new(rpc_router, vec![tool_weather]);

	// -- Execute question with conv
	let response = conv::send_user_msg(oa_client, ai_tools, question).await?;

	println!("\nFinal answer:\n\n{response}");

	Ok(())
}
