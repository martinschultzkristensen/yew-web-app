//components/atoms/dancer.rs
//Purpose of code: Create a dancer struct which can be used in src/components/data/choreography_data.rs
use yew::prelude::*;
#[derive(Clone)]
pub struct Dancer {
    pub id: usize,
    pub image: String,
    pub name: String,
    pub strength: u8,
    pub flexibility: u8,
}

impl Dancer {
    fn new(id: usize, image: String, name: String, strength: u8, flexibility: u8) -> Self {
        Self {
            id,
            image,
            name,
            strength,
            flexibility,
        }
    }
      // Method to validate dancer attributes
      pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Dancer name cannot be empty".to_string());
        }
        if self.strength > 100 || self.flexibility > 100 {
            return Err("Strength and flexibility must be between 0 and 100".to_string());
        }
        Ok(())
    }
}
// Dancer Management Struct
#[derive(Clone)]
pub struct DancerManager {
    dancers: Vec<Dancer>,
    next_id: usize,
}

impl DancerManager {
    pub fn new() -> Self {
        Self {
            dancers: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_dancer(&mut self, image: String, name: String, strength: u8, flexibility: u8) -> Result<usize, String> {
        let dancer = Dancer::new(self.next_id, image, name, strength, flexibility);
        
        // Validate the dancer before adding
        dancer.validate()?;
        
        self.dancers.push(dancer);
        let added_id = self.next_id;
        self.next_id += 1;
        
        Ok(added_id)
    }

    pub fn remove_dancer(&mut self, id: usize) -> Option<Dancer> {
        if let Some(index) = self.dancers.iter().position(|d| d.id == id) {
            Some(self.dancers.remove(index))
        } else {
            None
        }
    }

    pub fn get_dancers(&self) -> &Vec<Dancer> {
        &self.dancers
    }

    pub fn update_dancer(&mut self, id: usize, image: Option<String>, name: Option<String>, strength: Option<u8>, flexibility: Option<u8>) -> Result<(), String> {
        if let Some(dancer) = self.dancers.iter_mut().find(|d| d.id == id) {
            if let Some(new_image) = image {
                dancer.image = new_image;
            }
            if let Some(new_name) = name {
                dancer.name = new_name;
            }
            if let Some(new_strength) = strength {
                dancer.strength = new_strength;
            }
            if let Some(new_flexibility) = flexibility {
                dancer.flexibility = new_flexibility;
            }

            // Re-validate the updated dancer
            dancer.validate()?;
            
            Ok(())
        } else {
            Err("Dancer not found".to_string())
        }
    }
}

// Default implementation for initializing with some dancers
impl Default for DancerManager {
    fn default() -> Self {
        let mut manager = Self::new();
        
        // Add some initial dancers
        let _ = manager.add_dancer(
            "path/to/default/image1.jpg".to_string(), 
            "John Dancer".to_string(), 
            8, 
            7
        );
        let _ = manager.add_dancer(
            "path/to/default/image2.jpg".to_string(), 
            "Emma Performer".to_string(), 
            6, 
            9
        );
         
        manager
    }
}

#[derive(Properties, PartialEq)]
pub struct StatBarProps {
    pub value: u8,
    pub label: String,
}

#[function_component(StatBar)]
fn stat_bar(props: &StatBarProps) -> Html {
    let percentage: u8 = props.value * 10;
    
    html! {
        <div class="stat-container" style={format!("--stat-percentage: {}%", percentage)}>
            <span class="stat-label">{&props.label}</span>
            <div class="stat-bar-border">
                <div class="stat-bar-fill" style={format!("width: {}%", percentage)}></div>
            </div>
            // <span class="stat-value">{props.value}</span>
        </div>
    }
}


#[derive(Properties, PartialEq)]
pub struct DancerCardProps {
    pub image: String,
    pub name: String,
    pub strength: u8,
    pub flexibility: u8,
}

#[function_component(DancerCard)]
pub fn dancer_card(props: &DancerCardProps) -> Html {
    html! {
        <div class="info-section-container">
            <img src={props.image.clone()} alt={format!("Image of {}", props.name)} />
            <div class="name-and-stats-container">
            <p>{&props.name}</p>
                <StatBar value={props.strength} label="strength" />
                <StatBar value={props.flexibility} label="flexibility" />
            </div>
        </div>
    }
}