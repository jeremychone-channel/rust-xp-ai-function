use crate::chat;
use crate::model::ModelManager;
use async_openai::types::ChatCompletionTool;
use rpc_router::{router_builder, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};

pub(super) fn router_builder() -> RouterBuilder {
	router_builder![get_weather]
}

pub(super) fn chat_tools() -> crate::Result<Vec<ChatCompletionTool>> {
	let tool_weather = chat::tool_fn_from_type::<GetWeatherParams>()?;

	Ok(vec![tool_weather])
}

/// # get_weather
/// get the weather for a city
#[allow(unused)] // Will be passthrough API
#[derive(Debug, Deserialize, RpcParams, schemars::JsonSchema)]
struct GetWeatherParams {
	/// The city and state, e.g. San Francisco, CA
	location: String,
	/// The full country name of the city
	country: String,
	/// Unit respecting the country of the city
	unit: TempUnit,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, RpcParams)]
enum TempUnit {
	Celcius,
	Fahrenheit,
}

#[derive(Serialize)]
struct Weather {
	temperature: f64,
	unit: TempUnit,
	humidity_rh: f32,
}

async fn get_weather(
	_mm: ModelManager,
	params: GetWeatherParams,
) -> Result<Weather, String> {
	Ok(Weather {
		temperature: 30.,
		unit: params.unit,
		humidity_rh: 0.3,
	})
}
