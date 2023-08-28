use hecs::*;
use hecs_systems::*;

fn main(){
    let mut world = World::new();

    none_comps.into_system().execute(&mut world);
    one_comp.into_system().execute(&mut world);
}


fn none_comps(){

}

fn one_comp(a: &i32){

}