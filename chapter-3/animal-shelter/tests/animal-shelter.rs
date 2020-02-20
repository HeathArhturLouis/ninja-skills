use animal_shelter as ansh;

#[test]
fn test_new() {
    let shelter = ansh::AnimalShelter::new();
    assert_eq!(shelter.n_cats(), 0);
    assert_eq!(shelter.n_dogs(), 0);
    assert_eq!(shelter.n_animals(), 0);
}

#[test]
fn test_enqueue_size() {
    let mut shelter = ansh::AnimalShelter::new();
    shelter.enqueue(ansh::Animal::Cat(ansh::Cat {
        name: "Maukit".to_string(),
        declawed: false,
    }));
    assert_eq!(shelter.n_cats(), 1);
    assert_eq!(shelter.n_dogs(), 0);
    assert_eq!(shelter.n_animals(), 1);

    shelter.enqueue(ansh::Animal::Dog(ansh::Dog {
        name: "Pippin".to_string(),
        mass: 4000,
    }));
    assert_eq!(shelter.n_cats(), 1);
    assert_eq!(shelter.n_dogs(), 1);
    assert_eq!(shelter.n_animals(), 2);
}

#[test]
fn test_retrieve_cat() {
    let mut shelter = ansh::AnimalShelter::new();
    shelter.enqueue(ansh::Animal::Cat(ansh::Cat {
        name: "A".to_string(),
        declawed: true,
    }));
    shelter.enqueue(ansh::Animal::Cat(ansh::Cat {
        name: "B".to_string(),
        declawed: false,
    }));
    assert_eq!(
        shelter.dequeue_cat(),
        Some(ansh::Cat {
            name: "A".to_string(),
            declawed: true,
        })
    );
    assert_eq!(
        shelter.dequeue_cat(),
        Some(ansh::Cat {
            name: "B".to_string(),
            declawed: false,
        })
    );
    assert_eq!(shelter.dequeue_cat(), None);
}

#[test]
fn test_retrieve_dog() {
    let mut shelter = ansh::AnimalShelter::new();
    shelter.enqueue(ansh::Animal::Dog(ansh::Dog {
        name: "A".to_string(),
        mass: 10_000,
    }));
    shelter.enqueue(ansh::Animal::Dog(ansh::Dog {
        name: "B".to_string(),
        mass: 20_000,
    }));
    assert_eq!(
        shelter.dequeue_dog(),
        Some(ansh::Dog {
            name: "A".to_string(),
            mass: 10_000,
        })
    );
    assert_eq!(
        shelter.dequeue_dog(),
        Some(ansh::Dog {
            name: "B".to_string(),
            mass: 20_000,
        })
    );
    assert_eq!(shelter.dequeue_dog(), None);
}

#[test]
fn test_retrieve_mixed() {
    let mut shelter = ansh::AnimalShelter::new();
    shelter.enqueue(ansh::Animal::Dog(ansh::Dog {
        name: "0".to_string(),
        mass: 12_500,
    }));
    shelter.enqueue(ansh::Animal::Cat(ansh::Cat {
        name: "1".to_string(),
        declawed: true,
    }));
    shelter.enqueue(ansh::Animal::Cat(ansh::Cat {
        name: "2".to_string(),
        declawed: false,
    }));
    shelter.enqueue(ansh::Animal::Dog(ansh::Dog {
        name: "3".to_string(),
        mass: 20_000,
    }));

    assert_eq!(
        shelter.dequeue(),
        Some(ansh::Animal::Dog(ansh::Dog {
            name: "0".to_string(),
            mass: 12_500,
        }))
    );
    assert_eq!(
        shelter.dequeue(),
        Some(ansh::Animal::Cat(ansh::Cat {
            name: "1".to_string(),
            declawed: true,
        }))
    );
    assert_eq!(
        shelter.dequeue_dog(),
        Some(ansh::Dog {
            name: "3".to_string(),
            mass: 20_000,
        })
    );
    assert_eq!(
        shelter.dequeue(),
        Some(ansh::Animal::Cat(ansh::Cat {
            name: "2".to_string(),
            declawed: false,
        }))
    );
}
