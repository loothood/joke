use rand::{ thread_rng, Rng };

#[derive(Debug)]
pub struct RandomData<T> {
    pub value : T,
}

impl <T : Clone> RandomData<T> {

    pub fn new(vector : &[T]) -> Self {
        let mut rng = thread_rng();
        let value = rng.choose(&vector).unwrap();
        RandomData { value : value.clone() }
    }
}