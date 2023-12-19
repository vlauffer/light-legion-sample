use std::io;
use legion::{*, world::SubWorld, systems::CommandBuffer};
use rnglib::{RNG, Language};
use rand::{Rng, random};

#[derive(Clone)]
struct Person {
	name: String,
	coolness: i32
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

struct WantsToTalkTo{
    entities: Vec<Entity>
}

#[system]
#[read_component(Person)]
pub fn wants_to_talk_to( world: &SubWorld, commands: &mut CommandBuffer){
	let mut entity_person = <(Entity, &Person)>::query();
    let mut sorted: Vec<(&Entity, &Person)> = entity_person
        .iter(world)
        .map(|(e,p)| (e, p))
        .collect();
    sorted.sort_by(|a,b| a.1.coolness.cmp(&b.1.coolness));    
    entity_person.iter(world).for_each(|(e, p)|{
        let current_coolness = p.coolness;
        let mut talk_to_vec: Vec<Entity> = Vec::new();
        // Could b-search but too lazy
        for  (next_e,next_p) in sorted.iter(){
            if *next_e == e {
                continue;
            }
            if current_coolness <= next_p.coolness{
                talk_to_vec.push(**next_e);
            }
        }
        commands.add_component(*e, WantsToTalkTo{entities: talk_to_vec});
    });
}

#[system(for_each)]
#[read_component(Person)]
pub fn announce_conversation_interest(world: &SubWorld, e: &Entity, p: &Person, w: &WantsToTalkTo){
    for(entity) in &w.entities{
        let included_entry = world.entry_ref(*entity);
        if let Ok(found_entry) = included_entry{
            let potential_person = found_entry.get_component::<Person>().expect("fuck");
            println!("{} wants to talk to {}", p.name, potential_person.name);
        }
    }
}

fn main() {
    let mut world = World::default();
    for _ in 0..rand::thread_rng().gen_range(3..5){
        world.push((Person::new(),));
    }
    let mut schedule_builder = Schedule::builder();
    schedule_builder
        .add_system(wants_to_talk_to_system())
        .flush()
        .add_system(announce_conversation_interest_system())
        .flush();
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
