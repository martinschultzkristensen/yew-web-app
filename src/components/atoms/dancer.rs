//components/atoms/dancer.rs
//Purpose of code: Create a dancer struct which can be used in src/components/data/choreography_data.rs
struct Dancer {
    image: String,
    name: String,
    strength: u8,
    flexibility: u8,
}

impl Dancer {
    fn new(image: String, strength: u8, flexibility: u8) -> Self {
        Self {
            image,
            name,
            strength,
            flexibility,
        }
    }

    fn display_stats(&self) {
        println!("Strength: {}", self.strength);
        println!("Flexibility: {}", self.flexibility);
    }
}

const ALEX: Dancer = Dancer {
    image: "alex.png".to_string(),
    name: "Alex".to_string(),
    strength: 8,
    flexibility: 7,
};

const MARTIN: Dancer = Dancer {
    image: "martin.png".to_string(),
    name: "Martin".to_string(),
    strength: 10,
    flexibility: 5,
};