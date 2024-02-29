// use std::any::Any;
// use std::cell::RefCell;
// use std::rc::Rc;

use ecs_library::World;



#[test]
fn create_entity() -> anyhow::Result<()> {
    
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world.create_entity()
        .with_component(Location(42.0, 24.0))?
        .with_component(Size(10.0))?;

    Ok(())

}

#[test]
fn query_for_entities() -> anyhow::Result<()> {
     
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world.create_entity()
        .with_component(Location(42.0, 24.0))?
        .with_component(Size(10.0))?;

    world.create_entity()
        .with_component(Size(11.0))?;

    world.create_entity()
        .with_component(Location(43.0, 25.0))?;

    world.create_entity()
        .with_component(Location(44.0, 26.0))?
        .with_component(Size(12.0))?;

    let query = world.query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    let locations = &query.1[0];
    let sizes= &query.1[1];
    let indexes = &query.0;

    assert_eq!(locations.len(), sizes.len());
    assert_eq!(locations.len() , 2);
    assert_eq!(locations.len() , indexes.len());

    let borrowed_first_location = locations[0].borrow();
    let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();
    assert_eq!(first_location.0, 42.0);
    let borrowed_first_szie = sizes[0].borrow();
    let first_size = borrowed_first_szie.downcast_ref::<Size>().unwrap();
    assert_eq!(first_size.0, 10.0);


    let borrowed_second_location = locations[1].borrow();
    let second_location = borrowed_second_location.downcast_ref::<Location>().unwrap();
    assert_eq!(second_location.0, 44.0);
    let mut borrowed_second_szie = sizes[1].borrow_mut();
    let second_size = borrowed_second_szie.downcast_mut::<Size>().unwrap();
    second_size.0 += 1.0;
    assert_eq!(second_size.0, 13.0);

    Ok(())
}

#[test]
fn deleted_component_from_entitiy() -> anyhow::Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world.create_entity()
        .with_component(Location(10.0, 11.0))?
        .with_component(Size(10.0))?;

    world.create_entity()
        .with_component(Location(20.0, 21.0))?
        .with_component(Size(20.0))?;

    world.delete_component_by_entity_id::<Location>(0)?;

    let query = world.query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();
    let indexes = &query.0;

    assert_eq!(indexes.len(), 1);
    assert_eq!(indexes[0], 1);

    Ok(())
}


#[test]
fn add_component_to_entity() -> anyhow::Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world.create_entity()
    .with_component(Location(10.0, 11.0))?;

    world.add_component_to_entity_by_id(Size(20.0), 0)?;

    let query = world.query()
        .with_component::<Location>()?
        .with_component::<Size>()?
        .run();

    let indexes = &query.0;
    assert_eq!(indexes.len(), 1);
    Ok(())
}


#[test]
fn deleting_an_entity() -> anyhow::Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();
    world.create_entity()
        .with_component(Location(10.0, 11.0))?;
    world.create_entity()
        .with_component(Location(20.0, 21.0))?;        

    world.delete_entity_by_id(0)?;

    let query = world.query()
        .with_component::<Location>()?       
        .run();
    let indexes = &query.0;
    assert_eq!(indexes.len(), 1);

    let locations = &query.1[0];
    let borrowed_location = locations[0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();
    assert_eq!(location.0, 20.0);
    assert_eq!(location.1, 21.0);
    
    world.create_entity()
        .with_component(Location(30.0, 35.0))?;      
    let query = world.query()
        .with_component::<Location>()?       
        .run();
    let indexes = &query.0;
    assert_eq!(indexes.len(), 2);

    let locations = &query.1[0];
    let borrowed_location = locations[0].borrow();
    let location = borrowed_location.downcast_ref::<Location>().unwrap();
    assert_eq!(location.0, 30.0);
    assert_eq!(location.1, 35.0);

    Ok(())
}

struct Location(pub f32, pub f32);
struct Size(pub f32);