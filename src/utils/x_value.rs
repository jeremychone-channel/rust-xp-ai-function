use crate::Result;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait XValue {
	fn x_take<T: DeserializeOwned>(&mut self, name: &str) -> Result<T>;
}

impl XValue for Value {
	fn x_take<T: DeserializeOwned>(&mut self, name: &str) -> Result<T> {
		let value = self
			.get_mut(name)
			.map(Value::take)
			.ok_or(format!("No property '{name}' found."))?;

		let value: T = serde_json::from_value(value)?;
		Ok(value)
	}
}
