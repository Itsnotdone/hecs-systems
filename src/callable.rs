use hecs::{World, Query};

pub trait Callable<P>{
    fn run(&mut self, world: &mut World);
}

impl <F>Callable<()> for F
where
    F: FnMut()
{
    fn run(&mut self, world: &mut World) {
        self()
    }
}

impl <F, T0>Callable<(T0, )> for F
where
    for<'a>F: FnMut(T0::Item<'a>),
    T0: Query
{
    fn run(&mut self, world: &mut World) {
        for (_, t0) in world.query_mut::<T0>(){
            self(t0)
        }
    }
}