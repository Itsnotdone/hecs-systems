use crate::{Callable, System};

pub trait IntoSystem<P>
where
    Self: Callable<P> + Sized
{
    fn into_system(self) -> System<Self, P>;
}

impl <F>IntoSystem<()> for F
where
    F: Callable<()>
{
    fn into_system(self) -> System<Self, ()> {
        System::new(self)
    }
}

impl <F, T0>IntoSystem<(T0, )> for F
where
    F: Callable<(T0, )>
{
    fn into_system(self) -> System<Self, (T0,)> {
        System::new(self)
    }
}