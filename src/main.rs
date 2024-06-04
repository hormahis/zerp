// #![deny(warnings)]

use serde_json::{
	from_reader as jfrom_reader,
};
use std::{
	fs::File,
	io::{BufReader, read_to_string,},
	path::Path,
};
use rhai::{
	Engine,
	Scope,
};

mod models;
use models::{
	World,
};


fn main() {

	let file = File::open(Path::new("./world.json")).unwrap();
	let world_reader = BufReader::new(file);
	
	let file = File::open(Path::new("./script.rhai")).unwrap();
	let script_reader = read_to_string(BufReader::new(file)).unwrap();


	let mut world: World = jfrom_reader(world_reader).unwrap();
	world.init();


	let mut scope = Scope::new();
	scope
		.push("world", world);

	let mut engine = Engine::new();
	engine
		.build_type::<models::Object>()
		.build_type::<models::Person>()
		.build_type::<World>()
	// 	.register_fn("new_position", |level: i32, lat: i32, lon: i32| {LevelPosition::from((level, lat, lon))})
	;

	let r = engine.eval_with_scope::<()>(&mut scope, &script_reader);
	println!("eval: {r:#?}");

}
