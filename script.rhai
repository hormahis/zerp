print(world);

for ref in world.discover(0, 0) {
	print(`## ref ${ref}`);
	print(world.object(ref));
}