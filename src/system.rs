use std::marker::PhantomData;
use hecs::World;

pub struct System<F,P>{
    function: F,
    marker: PhantomData<P>
}

impl <F,P>System<F,P>{
    pub fn new(function: F) -> Self{
        Self { 
            function: function, 
            marker: PhantomData 
        }
    }
}


pub trait SystemExecuter{
    fn execute(&mut self, world: &mut World);
}

impl <F,P>SystemExecuter for System<F,P>{
    fn execute(&mut self, world: &mut World) {
        
    }
}