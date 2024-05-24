// link: https://platform.openai.com/docs/models/overview

pub const MODEL: &str = MODEL_3_TURBO;

// -- GPT 4 Turbo

// Should use this one if want to use 4
pub const MODEL_4_O: &str = "gpt-4o";

// Should be legacy model at this point
// Typically point to the latest (as of 2024-03-13 - "gpt-4-0125-preview")
pub const MODEL_4_TURBO: &str = "gpt-4-turbo-preview";

// -- GPT 3.5 Turbo

// Typically point to the latest (as of 2024-03-13 - "gpt-3.5-turbo-0125")
pub const MODEL_3_TURBO: &str = "gpt-3.5-turbo";
