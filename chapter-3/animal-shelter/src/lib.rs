use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Cat {
    pub name: String,
    pub declawed: bool,
}

#[derive(Debug, PartialEq)]
pub struct Dog {
    pub name: String,
    pub mass: i32, // in grams.
}

#[derive(Debug, PartialEq)]
pub enum Animal {
    Cat(Cat),
    Dog(Dog),
}

pub struct AnimalShelter {
    time: u64,
    cats: VecDeque<(Cat, u64)>,
    dogs: VecDeque<(Dog, u64)>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        Self {
            time: 0,
            cats: VecDeque::new(),
            dogs: VecDeque::new(),
        }
    }

    pub fn n_cats(&self) -> usize {
        self.cats.len()
    }

    pub fn n_dogs(&self) -> usize {
        self.dogs.len()
    }

    pub fn n_animals(&self) -> usize {
        self.n_cats() + self.n_dogs()
    }

    pub fn enqueue(&mut self, animal: Animal) {
        match animal {
            Animal::Cat(cat) => self.cats.push_back((cat, self.time)),
            Animal::Dog(dog) => self.dogs.push_back((dog, self.time)),
        }
        self.time += 1;
    }

    pub fn dequeue_cat(&mut self) -> Option<Cat> {
        self.cats.pop_front().map(|(c, _)| c)
    }

    pub fn dequeue_dog(&mut self) -> Option<Dog> {
        self.dogs.pop_front().map(|(d, _)| d)
    }

    pub fn dequeue(&mut self) -> Option<Animal> {
        match (self.cats.front(), self.dogs.front()) {
            (None, None) => None,
            (Some((_, _)), None) => self.cats.pop_front().map(|(c, _)| Animal::Cat(c)),
            (None, Some((_, _))) => self.dogs.pop_front().map(|(d, _)| Animal::Dog(d)),
            (Some((_, cat_time)), Some((_, dog_time))) => {
                if cat_time <= dog_time {
                    self.cats.pop_front().map(|(c, _)| Animal::Cat(c))
                } else {
                    self.dogs.pop_front().map(|(d, _)| Animal::Dog(d))
                }
            }
        }
    }
}
