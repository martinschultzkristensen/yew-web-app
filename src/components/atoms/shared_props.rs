//src/components/atoms/shared_props.rs
//these shared props are used in #[function_component(MainMenu / LoadScreenVideo / IntroScreen / ChoreoVideo / AboutChoreo)]
use std::rc::Rc;
use yew::prelude::*;
use crate::Config;

#[derive(Properties, PartialEq)]
pub struct AppConfigProps {
#[prop_or_default]
    pub choreo_number: usize, //only used in about_choreo.rs by variable choreography_data
    pub config: Rc<Config>,
}