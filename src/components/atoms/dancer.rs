//components/atoms/dancer.rs
//Purpose of code: Create a dancer struct which can be used in src/components/data/choreography_data.rs
#[derive(Clone)]
pub struct Dancer {
    pub image: String,
    pub name: String,
    pub strength: u8,
    pub flexibility: u8,
}

impl Dancer {
    fn new(image: String, name: String, strength: u8, flexibility: u8) -> Self {
        Self {
            image,
            name,
            strength,
            flexibility,
        }
    }
}