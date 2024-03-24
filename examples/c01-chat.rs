use async_openai::types::CreateChatCompletionRequest;
use xp_ai_function::oa_client::new_oa_client;
use xp_ai_function::{chat, gpts};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// -- Init AI Client
	let oa_client = new_oa_client()?;
	let chat_client = oa_client.chat();
	let model = gpts::MODEL.to_string();

	// -- User question
	let question = "Why is the sky red? (be concise)";

	// -- Build messages
	let messages = vec![chat::user_msg(question)?];

	// -- Exec Chat Request
	let msg_req = CreateChatCompletionRequest {
		model,
		messages,
		..Default::default()
	};
	let chat_response = chat_client.create(msg_req).await?;
	let first_choice = chat::first_choice(chat_response)?;

	// -- Display response
	let response = first_choice.message.content.ok_or("No message content?")?;

	println!("\nResponse:\n\n{response}");

	Ok(())
}
