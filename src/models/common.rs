use serde::Deserialize;
use serde_json::{
	Value,
};
use std::collections::HashMap;


#[derive(Debug, Deserialize, Clone, Default)]
pub struct LevelPosition {
	pub layer: i32,
	pub lat: i32,
	pub lon: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelObjectAttributes {
	pub name: Option<String>,
	pub title: Option<String>,

}

#[derive(Debug, Deserialize, Clone)]
pub struct LevelObjectClass {

	#[serde(flatten)]
	pub attr: LevelObjectAttributes,
}



#[derive(Debug, Deserialize)]
pub struct LevelIntermediateObject {
	pub class: String,
	pub position: LevelPosition,
	pub id: i32,

	#[serde(flatten)]
	pub attr: LevelObjectAttributes,
}


#[derive(Debug, Deserialize)]
pub struct LevelIntermediateObjects {
	pub name: String,
	pub classes: HashMap<String, LevelObjectClass>,
	pub objects: Vec<LevelIntermediateObject>,
}






impl LevelPosition {
	pub fn as_coordinates(&self) -> (i32, i32, i32) {
		(self.layer, self.lat, self.lon)
	}
}

impl LevelObjectAttributes {
	pub fn merge(base: &LevelObjectAttributes, extra: &LevelObjectAttributes) -> HashMap<String, rhai::Dynamic> {
		HashMap::from([
			("name".into(), rhai::Dynamic::from(extra.name.clone().unwrap_or_else(|| base.name.clone().unwrap_or_default()))),
			("title".into(), rhai::Dynamic::from(extra.title.clone().unwrap_or_else(|| base.title.clone().unwrap_or_default()))),
		])
	}
}


/// Tuple(3x isize) --> LevelPosition
impl From<&(i32, i32, i32)> for LevelPosition {
	fn from(p: &(i32, i32, i32)) -> LevelPosition {
		Self {
			layer: p.0,
			lat: p.1,
			lon: p.2
		}
	}
}
impl From<(i32, i32, i32)> for LevelPosition {
	fn from(p: (i32, i32, i32)) -> LevelPosition { LevelPosition::from(&p) }
}