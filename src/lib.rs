use std::any::Any;


mod resources;

use resources::Resource;


#[derive(Default, Debug)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }


    /// This is for adding a resource 
    /// The type of the reosurces must be added in so that we can find it
    /// ```
    /// use::ecs_library::World;
    /// let mut world = World::new();
    /// world.add_resouce(10_u32);    
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 10);
    /// ```
    pub fn add_resouce(&mut self, resouce_data: impl Any) {
        self.resources.add(resouce_data);

    }

    /// Query for a resource and get a reference to it.
    /// The type of the reosurces must be added in so that we can find it
    /// ```
    /// use::ecs_library::World;
    /// let mut world = World::new();
    /// world.add_resouce(10_u32);    
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 10);
    /// ```
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a reference to it.
    /// The type of the reosurces must be added in so that we can find it
    /// ```
    /// use::ecs_library::World;
    /// let mut world = World::new();
    /// world.add_resouce(10_u32);
    /// {   
    ///    let resource = world.get_resource_mut::<u32>().unwrap();
    ///    *resource += 1;
    /// }
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 11);
    /// ```
    pub fn get_resource_mut<T:Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// This is  for remove the resource.
    /// Thne type of the reosurces must be added in so that we can find it
    /// ```
    /// use::ecs_library::World;
    /// let mut world = World::new();
    /// world.add_resouce(10_u32);
    /// world.delete_resource::<u32>();
    /// let deleted_resource = world.get_resource::<u32>();
    /// assert!(deleted_resource.is_none());
    /// ```    
    pub fn delete_resource<T:Any>(&mut self) {
        self.resources.remove::<T>();
    }
}


#[cfg(test)]
mod tests {}
