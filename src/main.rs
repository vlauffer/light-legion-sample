use std::{io, collections::HashMap};

use legion::{*, world::SubWorld, systems::CommandBuffer};
use rnglib::{RNG, Language};
use rand::{Rng, random};

struct Person {
	name: String,
    coolness: i32
}

struct Likes_Talking_to {
    ent: Entity
}

impl Person {
    fn new() -> Self{
        Self { 
            name: RNG::try_from(&Language::Elven)
                .expect("random name could not be generated")
                .generate_name(),
            coolness: rand::thread_rng().gen_range(1..11), 
        }
    }
}

#[system]
#[read_component(Person)]
#[write_component(Person)]
#[read_component(Entity)]
#[write_component(Entity)]
fn find_people_to_talk_to( world: &mut SubWorld, commands: &mut CommandBuffer){
	let mut entities = <(Entity, &Person)>::query();
	let mut just_persons = <Read<Person>>::query();
	let mut pv: Vec<i32> = Vec::new();
	let mut counter: i32 = 0;
    for (e,p) in entities.iter(world) {
        // println!("finding people for {} to talk to", p.name);
		pv.push( p.coolness);
		// counter += 1;
    }
	pv.sort();
	let mut coll: Vec<Entity> = Vec::new();
	let mut rm: Vec<(&Entity, &Person)>= entities.iter(world).map(|(e,p)| (e, p)).collect::<Vec<_>>();
	rm.sort_by(|a, b| a.1.coolness.cmp(&b.1.coolness));
	let mut correlation_map: HashMap<&Entity, &Entity> = HashMap::new();
	for i in 0..rm.len(){
		if i >= rm.len() -1 {
			break;
		}
		correlation_map.insert(rm[i].0, rm[i+1].0);
		let mut cp = rm[i];
		let vv = pv[i];
		// let mut mutable_ent = world.entry(entity)
		
		// println!("{} should be {}", cp.1.coolness, vv);
		world.entry_ref(*rm[i].0);
	}
	// let (mut left, right) = world.split::<&mut Entity>();
	for (k, v) in correlation_map {
		// let e_ref = world
		// 	.entry_mut(*k)
		// 	.unwrap()
		// 	.get_component_mut::<Person>();
		// if let Ok(person_from_ent) = e_ref {

		// }

		println!("asdf");

	}

	// for (e,p) in entities.iter_mut(world){
	// 	let real_ent = world.entry_mut(*e);
	// }	

	// for (e, p) in rm {
	// 	println!("{} is a {}", p.name, p.coolness);
	// 	// println!("{} wwww", pv[])
	// }
		// .sort_by(|a, b| 
		// 	a.1.coolness.cmp(&b.1.coolness).then_with(|| a.1.name.cmp(&b.1.name)));

	// let rv:Vec<&Entity> = rm.collect();
	// let pp = rm.
	// for(e, p) in entities.iter_mut(world). {
	// 	coll.push(*e);
	// }
}

fn main() {
    let mut world = World::default();
    for _ in 0..rand::thread_rng().gen_range(2..11){
        world.push((Person::new(),));
    }
    let mut schedule_builder = Schedule::builder();
    schedule_builder.add_system(find_people_to_talk_to_system());
    let mut schedule = schedule_builder.build();
    let mut resources = Resources::default();
    print_world(&world);
    loop{
		println!("Hit return to step forward. Hit q and return to quit");
        schedule.execute(&mut world, &mut resources);
        let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
        
		match input.trim() {
		  "q" => {
			println!("Goodbye!");
			break;
		  }
		  _ => println!("Looping..."),
		}
    }
}

fn print_world(world: &World){
    let mut query = <&Person>::query();
    for p in query.iter(world){
        match p.coolness > 7 {
            true => println!("{} is so cool! Cool level {}", p.name, p.coolness),
            false => println!("{} is lame :( Cool level {}", p.name, p.coolness),
        }
	}
}
