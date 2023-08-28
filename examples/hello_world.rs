use hecs::*;
use hecs_systems::*;

fn main(){
    let mut world = World::new();

    world.spawn((10,));

    let mut schedule = Schedule::new();

    schedule.add_system(none_comps);
    schedule.add_system(one_comp);

    schedule.execute(&mut world);
}


fn none_comps(){
    println!("hello world")
}

fn one_comp(number: &i32){
    println!("number: {}", number);
}