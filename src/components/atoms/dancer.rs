//components/atoms/dancer.rs
//Purpose of code: Create a dancer struct which can be used in src/components/data/choreography_data.rs
use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct DancerData {
    pub image: String,
    pub name: String,
    pub strength: u8,
    pub flexibility: u8,
}

impl DancerData {
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
    pub dancer: DancerData,
}

#[function_component(DancerCard)]
pub fn dancer_card(props: &DancerCardProps) -> Html {
    let dancer = &props.dancer;
    html! {
        <div class="info-section-container">
            <img src={dancer.image.clone()} alt={format!("Image of {}", dancer.name)} />
            <div class="name-and-stats-container">
            <p>{&dancer.name}</p>
                <StatBar value={dancer.strength} label="strength" />
                <StatBar value={dancer.flexibility} label="flexibility" />
            </div>
        </div>
    }
}