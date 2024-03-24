use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	#[from]
	Custom(String),

	// -- Externals
	#[from]
	OpenAi(async_openai::error::OpenAIError),

	#[from]
	Json(serde_json::Error),

	#[from]
	RpcCall(rpc_router::CallError),
}

// region:    --- Froms

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Self::Custom(val.to_string())
	}
}

// endregion: --- Froms

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
