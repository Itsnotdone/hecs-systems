use hecs::World;

use crate::{SystemExecuter, IntoSystem};

pub struct Schedule{
    systems: Vec<Box<dyn SystemExecuter>>
}

impl Schedule{
    pub fn new() -> Self{
        Self { 
            systems: Vec::new() 
        }
    }

    pub fn add_system<F: IntoSystem<P> + 'static,P: 'static>(&mut self, system: F){
        self.systems.push(Box::new(system.into_system()))
    }

    pub fn execute(&mut self, world: &mut World){
        self.systems.iter_mut().for_each(|system| system.execute(world));
    }
}