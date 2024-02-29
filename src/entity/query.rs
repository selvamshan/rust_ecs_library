use std::any::{Any, TypeId};


use super::{Entites, Component};
use crate::entity::error::{Result, Error};

pub type QueryIndexes = Vec<usize>;
pub type QueryComponents = Vec<Vec<Component>>;

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entites,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {

    pub fn new(entities: &'a Entites)  -> Self {
        Self {
            entities, 
            map:0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id){
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else{
             Error::ComponetNotRegister("attempting use component that wasn't registerd".to_string());
        }
        Ok(self)
    }

    pub fn run(&self) -> (QueryIndexes, QueryComponents) {
        let indexes = self.entities.map.iter().enumerate()
            .filter_map(|(idx, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        //dbg!(indexes);

        let mut result = vec![];

        for type_id in &self.type_ids {
            let components = self.entities.components.get(type_id).unwrap();
            let mut query_components = Vec::new();
            for index in &indexes {
                query_components.push(components[*index].as_ref().unwrap().clone());
            }
            result.push(query_components)
        }

        (indexes,result)
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?
            .with_component::<f32>()?;

        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);
        Ok(())
    }


    #[test]
    fn run_qurey() -> Result<()> {
        let mut entities = Entites::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        entities.create_entity().with_component(10_u32)?.with_component(20.2_f32)?;
        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(10.2_f32)?;
        entities.create_entity().with_component(15_u32)?.with_component(30.2_f32)?;

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?
            .with_component::<f32>()?;

        let query_result = query.run();
        let u32s = &query_result.1[0];
        let f32s = &query_result.1[1];
        let indexes = query_result.0;

        assert!(u32s.len() == f32s.len() && u32s.len() ==  indexes.len());
        assert_eq!(u32s.len(), 2);

        let borrowed_first_u32s = u32s[0].borrow();
        let first_u32s = borrowed_first_u32s.downcast_ref::<u32>().unwrap();
        assert_eq!(*first_u32s, 10);
        let borrowed_first_f32s = f32s[0].borrow();
        let first_size = borrowed_first_f32s.downcast_ref::<f32>().unwrap();
        assert_eq!(*first_size, 20.2);
    
    
        let borrowed_second_u32s = u32s[1].borrow();
        let second_u32s = borrowed_second_u32s.downcast_ref::<u32>().unwrap();
        assert_eq!(*second_u32s, 15);
        let borrowed_second_f32s = f32s[1].borrow();
        let second_size = borrowed_second_f32s.downcast_ref::<f32>().unwrap();
        assert_eq!(*second_size, 30.2);

        assert_eq!(indexes[0], 0);
        assert_eq!(indexes[1], 3);


        Ok(())
    }

}

