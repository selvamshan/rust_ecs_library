use std::any::{Any, TypeId};
use std::collections::HashMap;



#[derive(Default, Debug)]
pub struct Resource {
    data: HashMap<TypeId, Box<dyn Any>>,
}


impl Resource {
 
    pub fn add(&mut self, data:impl Any) {
        let type_id = data.type_id();

        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            data.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_mut<T:Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            data.downcast_mut()
        } else {
            None
        }
    }


    pub fn remove<T:Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }

}

#[cfg(test)] 
mod test {
    use super::*;

    struct WorldWidth(pub f32);

    #[test] 
    fn add_resouce() {
        let resources= initialize_resources();

        let stored_resource = resources
            .data
            .get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_with = stored_resource
            .downcast_ref::<WorldWidth>().unwrap();

        assert_eq!(extracted_world_with.0, 100.0)
    }

    #[test]
    fn get_resource() {

        let resources = initialize_resources();

        if let Some(extracted_world_with) = resources.get_ref::<WorldWidth>() {
            assert_eq!(extracted_world_with.0, 100.0);
        }
    }

    #[test] 
    fn get_resorces_mut() {
        let mut resources = initialize_resources();
        {
            let world_width: &mut WorldWidth = resources.get_mut::<WorldWidth>().unwrap();
            world_width.0 += 1.0;
        }
        
        let world_width = resources.get_ref::<WorldWidth>().unwrap();
        assert_eq!(world_width.0, 101.0);
    }

    #[test]
    fn remove_resource() {
        let mut resources = initialize_resources();
        resources.remove::<WorldWidth>();
        let world_with_type_id = TypeId::of::<WorldWidth>();
        assert!(!resources.data.contains_key(&world_with_type_id));

    }

    fn initialize_resources() -> Resource {
        let mut resoruces = Resource::default();
        let world_width = WorldWidth(100.0);        
        resoruces.add(world_width);
        resoruces
    }
}