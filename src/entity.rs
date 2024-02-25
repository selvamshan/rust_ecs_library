use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::{Error,Result};


#[derive(Default, Debug)]
pub struct Entites {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entites {
    pub fn register_component<T:Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components
            .iter_mut()
            .for_each(|(_key, componets)| componets.push(None));
        self
    }

    pub fn with_component(&mut self, data:impl Any) -> Result<&mut Self>{
        let type_id = data.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            let last_component = components
                .last_mut()
                .ok_or_else(|| Error::ComponentNotFound).unwrap();
            *last_component = Some(Rc::new(RefCell::new(data)));

        } else {
            Error::ComponetNotRegister("try to insert data for component that wasn't registerd".to_string());
        }
        
        Ok(self)
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


    struct Health(pub u32);
    struct Speed(pub f32);
}