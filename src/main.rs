// #![deny(warnings)]

use serde_json::{
	from_reader as jfrom_reader,
};
use std::{
	fs::File,
	io::BufReader,
	path::Path,
};
use rhai::{
	Engine,
	Scope,
};

mod models;
use models::{
	LevelPosition,
	LevelIntermediateObjects,
	WorldObject,
	WorldMap,
};

const SCRIPT: &str = r#"
print(`${world}`);
let obj = world.object(new_position(0, 1, 1), 10004);
print(`${obj}`);

print(`Title through attribute: "${obj.attribute("title")}"`);
print(`Name  through prop     : "${obj.name}"`);
"#;


fn main() {

	let file = File::open(Path::new("./level_map.json")).unwrap();
	let reader = BufReader::new(file);


	let inter_objects: LevelIntermediateObjects = jfrom_reader(reader).unwrap();
	let world = WorldMap::from(inter_objects);

	let mut scope = Scope::new();
	scope
		.push("world", world);

	let mut engine = Engine::new();
	engine
		.build_type::<LevelPosition>()
		.build_type::<WorldObject>()
		.build_type::<WorldMap>()
		.register_fn("new_position", |level: i32, lat: i32, lon: i32| {LevelPosition::from((level, lat, lon))})
	;

	let r = engine.eval_with_scope::<()>(&mut scope, SCRIPT);
	println!("eval: {r:?}");


}
