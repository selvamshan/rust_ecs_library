use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


pub mod query;
mod error;
pub use self::error::{Error,Result};

pub type Component = Rc<RefCell<dyn Any>>;
pub type Components = HashMap<TypeId, Vec<Option<Component>>>;


//pub use entity::Query;
//use self::query::Query;

#[derive(Default, Debug)]
pub struct Entites {
    components: Components,
    bit_masks:HashMap<TypeId, u32>,
    map: Vec<u32>,
    inserting_into_index: usize,
}

impl Entites {
    pub fn register_component<T:Any + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let bit_mask = 2_u32.pow(self.bit_masks.len() as u32);
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, bit_mask);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        if let Some((index, _)) = self.map.iter().enumerate().find(|(_index, mask)| **mask == 0){
            self.inserting_into_index = index;
        } else {
            self.components
                .iter_mut()
                .for_each(|(_key, componets)| componets.push(None));
            self.map.push(0);
            self.inserting_into_index = self.map.len() - 1;
        }
        self
    }

    pub fn with_component(&mut self, data:impl Any) -> Result<&mut Self>{
        let type_id = data.type_id();
        //let map_index = self.map.len() -1 ;
        let index = self.inserting_into_index;
        if let Some(components) = self.components.get_mut(&type_id) {
            let component = components
                .get_mut(index)
                .ok_or_else(
                    || Error::ComponentNotFound("component not created using entity creation".to_string())
                )?;
            *component = Some(Rc::new(RefCell::new(data)));

            let bit_mask = self.bit_masks.get(&type_id).unwrap();
            self.map[index] |= *bit_mask;

        } else {
            Error::ComponetNotRegister("try to insert data for component that wasn't registerd".to_string());
        }
        
        Ok(self)
    }

    pub fn get_bitmask(&self, type_id:&TypeId) -> Option<u32> {
       self.bit_masks.get(type_id).copied()
    }

    fn has_component(&self, index: usize, mask: u32) -> bool {
        self.map[index] & mask == mask
    }

    pub fn delete_component_by_entity_id<T:Any>(&mut self, index:usize) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id){
            mask
        } else {
            return Err(
                Error::ComponetNotRegister("attempting use component that wasn't registerd".to_string())
            );
        };
        if self.has_component(index, *mask) {
            self.map[index] ^= *mask;
        }

        Ok(())
    }

    pub fn add_component_to_entity_by_id(&mut self, data: impl Any, index:usize) -> Result<()> {
        let type_id = data.type_id();
        let mask = if let Some(mask) = self.bit_masks.get(&type_id){
            mask
        } else {
            return Err(
                Error::ComponetNotRegister("attempting use component that wasn't registerd".to_string())
            );
        };

        self.map[index] |= *mask;
        let components = self.components.get_mut(&type_id).unwrap();
        components[index] = Some(Rc::new(RefCell::new(data)));

        Ok(())
    }

    pub fn delete_entity_by_id(&mut self, index:usize) -> Result<()> {
        if let Some(map) = self.map.get_mut(index) {
            *map = 0;
        } else {
            return Err(Error::EntityDoesNotExist("attemting to delete an entity does not exits".to_string()));
        }
       
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use std::any::TypeId;
    use anyhow::Result;
    use super::*;

    #[test]
    fn register_an_entity() {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_component = entities
            .components.get(&type_id).unwrap();
        assert!(health_component.is_empty());

    }

    #[test]
    fn bitmask_updated_when_registrering_entities() {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let mask = entities
            .bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 1);

        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities
            .bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 2);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities.create_entity();
        let health = entities
            .components
            .get(&TypeId::of::<Health>())
            .unwrap();
        let speed = entities
            .components
            .get(&TypeId::of::<Speed>())
            .unwrap();

        assert!(health.len() == speed.len() && health.len() == 1);
        assert!(health[0].is_none() && speed[0].is_none());
        //dbg!(entities);
    }  


    #[test]
    fn with_component() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity()
            .with_component(Health(100))?            
            .with_component(Speed(25.0))?;

        let first_health = &entities
            .components
            .get(&TypeId::of::<Health>())
            .unwrap()[0].as_ref();
        let wrapped_health = first_health.unwrap();
        let borrowed_health = wrapped_health.borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();

        assert_eq!(health.0, 100);

        Ok(())
    }

    #[test]
    fn map_is_updated_when_creating_entities() -> Result<()>{
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity()
            .with_component(Health(100))?            
            .with_component(Speed(25.0))?;

        let entity_map = entities.map[0];
        assert_eq!(entity_map, 3);

        entities.create_entity()                    
            .with_component(Speed(25.0))?;
        let entity_map = entities.map[1];
        assert_eq!(entity_map, 2);

        Ok(())
    }

    #[test]
    fn test_delete_component_by_entity_id() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity()
            .with_component(Health(100))?            
            .with_component(Speed(25.0))?;

        entities.delete_component_by_entity_id::<Health>(0)?;

        assert_eq!(entities.map[0], 2);
        Ok(())
    }

    #[test]
    fn add_component_to_entity_by_id() ->Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity()
            .with_component(Health(100))?; 

        entities.add_component_to_entity_by_id(Speed(25.0), 0)?;
        assert_eq!(entities.map[0], 3);

        let speed_type_id = TypeId::of::<Speed>();
        let wrapped_speeds = entities.components.get(&speed_type_id).unwrap();
        let wrapped_speed = wrapped_speeds[0].as_ref().unwrap();
        let borrowed_speed = wrapped_speed.borrow();
        let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();
        assert_eq!(speed.0, 25.0);
        Ok(())
    }

    #[test]
    fn delete_an_entity_by_id() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();

        entities.create_entity()
            .with_component(Health(100))?; 
        entities.create_entity()
        .with_component(Health(150))?; 

        entities.delete_entity_by_id(0)?;

        assert_eq!(entities.map[0], 0);

        // let health_type_id = TypeId::of::<Health>();
        // let wrapped_health = entities.components.get(&health_type_id).unwrap();
        // let wrapped_health = wrapped_health[0].as_ref().unwrap();
        // let borrowed_health = wrapped_health.borrow();
        // let health = borrowed_health.downcast_ref::<Health>().unwrap();
        // assert_eq!(health.0, 150);
      
        Ok(())
    }

    #[test]
    fn created_entitites_are_inserter_into_deleted_entities_column() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<Health>(); 
        entities.create_entity().with_component(Health(100))?; 
        entities.create_entity().with_component(Health(150))?; 
        entities.delete_entity_by_id(0)?;
        assert_eq!(entities.map[0], 0);

        entities.create_entity().with_component(Health(25))?; 
        assert_eq!(entities.map[0], 1);
        let health_type_id = TypeId::of::<Health>();
        let wrapped_health = entities
            .components.get(&health_type_id).unwrap();
        let wrapped_health = wrapped_health[0].as_ref().unwrap();
        let borrowed_health = wrapped_health.borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();
        assert_eq!(health.0, 25);

        Ok(())
    }


    struct Health(pub u32);
    struct Speed(pub f32);
}