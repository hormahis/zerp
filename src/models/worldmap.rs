use super::common::{
	LevelPosition,
	LevelObjectClass,
	LevelIntermediateObjects,
};
use super::worldobject::{
	WorldObject,
};

use std::collections::HashMap;




#[derive(Debug, Clone)]
pub struct WorldMap {
	pub name: String,
	classes: HashMap<String, LevelObjectClass>,
	objects: HashMap<(i32, i32, i32), Vec<WorldObject>>,
}


/// From preloaded object
impl From<LevelIntermediateObjects> for WorldMap {
	fn from(o: LevelIntermediateObjects) -> Self {
		let mut objects = HashMap::<(i32, i32, i32), Vec<WorldObject>>::new();

		for obj in o.objects {
			let c = o.classes.get(&obj.class).unwrap();
			let pos = obj.position.as_coordinates();

			let res_obj = WorldObject::from((&obj, &c.attr));

			objects.entry(pos)
				.and_modify(|layer| layer.push(res_obj.clone()))
				.or_insert_with(|| {
					vec![res_obj]
				});
		}

		Self {
			name: o.name,
			classes: o.classes.clone(),
			objects
		}
	}
}

impl std::fmt::Display for WorldMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}


/// Worldmap methods themselves
impl WorldMap {

	pub fn get_at_pos(&self, position: &LevelPosition) -> Vec<WorldObject> {
		if let Some(pos) = self.objects.get(&position.as_coordinates()) {
			pos.to_vec()
		} else {
			vec![]
		}
	}

	pub fn show_class(&mut self, class: &str) -> LevelObjectClass {
		todo!("Show class must be implemented")
	}
	pub fn show_object(&mut self, pos: LevelPosition, obj: i32) -> rhai::Dynamic {
		let objects = self.get_at_pos(&pos);

		if !objects.is_empty() {
			if let Some(obj) = objects.into_iter().find(|o| o.id == obj) {
				return rhai::Dynamic::from(obj)
			}
		}
		
		rhai::Dynamic::from(None::<WorldObject>)
	}

	pub fn on_print(&mut self) -> String {
		format!("{}", self.name)
	}

}