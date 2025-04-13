use core::any::Any;

pub trait AnyClone: Any {
    fn clone_box(&self) -> Box<dyn AnyClone>;
}

impl Clone for Box<dyn AnyClone> {
    fn clone(&self) -> Box<dyn AnyClone> {
        self.clone_box()
    }
}

impl<T: Any + Clone> AnyClone for T {
    fn clone_box(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }
}