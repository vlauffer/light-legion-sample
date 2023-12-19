use crate::*;
struct WantsToDrink(bool);
#[system]
#[read_component(Person)]
#[write_component(Person)]
pub fn thirsty( world: &mut SubWorld, commands: &mut CommandBuffer){
	let mut entity_person = <(Entity, &Person)>::query();
	let more_ent: Vec<(Entity, Entity)>= entity_person
		.iter(world)
		.map(|(e, p)| (*e, *e))
		.collect();
	more_ent.iter().for_each(|(e, p)| {
		let is_here = world
			.entry_ref(*e)
			.unwrap()
			.get_component::<Person>()
			.is_ok();
        if let Ok( mut mutable_person) = world
            .entry_mut(*e)
            .unwrap()
            .get_component_mut::<Person>()
        {
            if mutable_person.coolness > 4{ // non-cool people stay inside and make bar sims in rust 🫠
                commands.add_component(*e, WantsToDrink(true));
            }
            mutable_person.coolness +=1;
        }
	});
}