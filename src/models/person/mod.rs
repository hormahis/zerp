use rhai::{
	CustomType,
	TypeBuilder,
};
use serde::Deserialize;



pub type PersonReference = String;

#[derive(Debug, Clone, Deserialize)]
pub struct Person {
	pub id: PersonReference,
	pub name: String,
	pub meta: PersonMeta,
	pub position: PersonPosition,
}


#[derive(Debug, Clone, Deserialize)]
pub struct PersonMeta {}


#[derive(Debug, Clone, Deserialize)]
pub struct PersonPosition {
	pub lon: f32,
	pub lat: f32,
	pub discovered: crate::models::object::ObjectReference,
	pub entered: crate::models::object::ObjectReference,
}


impl CustomType for Person {

	fn build(mut builder: TypeBuilder<Self>) {
		builder
			.with_name("Person")
		;

	}

}