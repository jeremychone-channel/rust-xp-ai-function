use xp_ai_function::conv;
use xp_ai_function::oa_client::new_oa_client;
use xp_ai_function::tools::new_ai_tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Init AI Client
	let oa_client = new_oa_client()?;

	// -- Get the AI Tools
	let ai_tools = new_ai_tools(None)?;

	// -- User questions
	let questions = &[
		"What is the weather in the California's best city and Paris?",
		"Why is the sky red? (be concise)",
		"what is the weather in Italy's capital",
	];

	// -- Execute question with conv
	for &question in questions {
		let response =
			conv::send_user_msg(oa_client.clone(), ai_tools.clone(), question)
				.await?;

		println!(
			r#"
== Question: {question}

{response}
		"#
		);
	}

	Ok(())
}
