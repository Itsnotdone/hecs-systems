use crate::{Callable, System};

pub trait IntoSystem<P>: Sized{
    fn into_system(self) -> System<Self, P>;
}

impl <F,P>IntoSystem<P> for F
where
    F: Callable<P>
{
    fn into_system(self) -> System<Self, P> {
        System::new(self)
    }
}