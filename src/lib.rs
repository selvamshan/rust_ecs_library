use std::any::Any;


mod resources;
mod entity;

use crate::entity::query::Query;
use entity::{Entites, Result};
use resources::Resource;


#[derive(Default, Debug)]
pub struct World {
    resources: Resource,
    entities: Entites,
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


    pub fn register_component<T:Any +'static >(&mut self) {
        self.entities.register_component::<T>();
    }


    pub fn create_entity(&mut self) -> &mut Entites {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query{
        Query::new(&self.entities)
    }

    pub fn delete_component_by_entity_id<T:Any>(&mut self, index:usize) -> Result<()> {
        self.entities.delete_component_by_entity_id::<T>(index)        
    }
  
    pub fn add_component_to_entity_by_id(&mut self, data: impl Any, index:usize)  -> Result<()> {
        self.entities.add_component_to_entity_by_id(data, index)
    }

    pub fn delete_entity_by_id(&mut self, index:usize) -> Result<()> {
        self.entities.delete_entity_by_id(index)?;

        Ok(())     
    }

}


#[cfg(test)]
mod tests {

}
