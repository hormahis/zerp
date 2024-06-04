use rhai::{
	CustomType,
	Dynamic,
	ImmutableString,
	TypeBuilder,
};
use serde::Deserialize;
use std::collections::HashMap;

use super::object::{
	Object,
	ObjectReference,
};
use super::person::{
	Person,
	PersonReference,
};

pub type ObjectRegistry = HashMap<ObjectReference, Object>;
pub type PersonRegistry = HashMap<PersonReference, Person>;


#[derive(Debug, Clone, Deserialize)]
pub struct World {
	objects: ObjectRegistry,

	#[serde(skip_deserializing, default)]
	objects_map: HashMap<(i32, i32), Vec<ObjectReference>>,

	persons: PersonRegistry,

	pub root: Object,
}


// Implement internal logic
impl World {
	pub fn init(&mut self) {

		// Update object locations
		for obj in self.objects.values() {
			println!("obj {} found", obj.id);
			let pos = (obj.location.lon, obj.location.lat);

			self.objects_map.entry(pos)
				.and_modify(|rfs| rfs.push(obj.id.clone()))
				.or_insert(vec![obj.id.clone()]);

			println!("  push to objects_map: {:?}", self.objects_map);
		}

	}
}


// Implement scripting functions
impl World {

	/// Extract object information from registry
	pub fn get_object(&mut self, reference: ImmutableString) -> Dynamic {

		if let Some(obj) = self.objects.get(reference.as_str()) {
			Dynamic::from(obj.clone())
		} else {
			Dynamic::from(())
		}

	}

	/// Get all the object references at certain location
	pub fn discover(&mut self, lon: i32, lat: i32) -> Vec<ImmutableString> {
		println!("asking {lon}:{lat}");

		if let Some(refs) = self.objects_map.get(&(lon, lat)) {
			println!("found at pos: {refs:?}");
			refs.iter().map(|r| r.clone().into()).collect()
		} else {
			vec![]
		}
	}

	/// Extract person information from registry
	pub fn get_person(&mut self, reference: ImmutableString) -> Dynamic {
		
		if let Some(person) = self.persons.get(reference.as_str()) {
			Dynamic::from(person.clone())
		} else {
			Dynamic::from(())
		}

	}

	pub fn on_print(&mut self) -> String {
		format!("Discovering {}...", self.root.name)
	}
}


impl CustomType for World {

	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("World")
			
			.with_fn("object", Self::get_object)
			.with_fn("person", Self::get_person)
			.with_fn("discover", Self::discover)

			.on_print(Self::on_print);
	}

}
