use serde_json::Value;
use std::collections::HashMap;
use crate::models::common::{
	LevelPosition,
	LevelIntermediateObject,
	LevelObjectAttributes,
};



#[derive(Debug, Clone)]
pub struct WorldObject {
	pub id: i32,
	class: String,
	position: LevelPosition,
	attr: HashMap<String, rhai::Dynamic>,
}


impl From<(&LevelIntermediateObject, &LevelObjectAttributes)> for WorldObject {
	fn from (o: (&LevelIntermediateObject, &LevelObjectAttributes)) -> Self {
		Self {
			id: o.0.id,
			class: o.0.class.clone(),
			position: o.0.position.clone(),
			attr: LevelObjectAttributes::merge(o.1, &o.0.attr)
		}
	}
}

impl std::fmt::Display for WorldObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:#?}")
	}
}

impl WorldObject {
	pub fn get_id(&mut self) -> i32 { self.id }
	
	pub fn get_name(&mut self) -> rhai::ImmutableString {
		if let Some(name) = self.attr.get("name") {
			name.clone().into_immutable_string().unwrap_or_else(|_| "".into())
		} else {
			"".into()
		}
	}
	
	pub fn get_title(&mut self) -> rhai::ImmutableString {
		if let Some(title) = self.attr.get("title") {
			title.clone().into_immutable_string().unwrap_or_else(|_| "".into())
		} else {
			"".into()
		}
	}
	
	pub fn get_attribute(&mut self, attr: rhai::ImmutableString) -> rhai::Dynamic {
		match self.attr.get(attr.as_str()) {
			Some(r) => rhai::Dynamic::from(r.clone()),
			_ => rhai::Dynamic::from(None::<Value>)
		}
	}
	
	pub fn on_print(&mut self) -> String {
		format!("Object {}, {:#?}", self.id, self.attr)
	}
}