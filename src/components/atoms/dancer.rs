//components/atoms/dancer.rs
//Purpose of code: Create a dancer struct which can be used in src/components/data/choreography_data.rs
use yew::prelude::*;
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