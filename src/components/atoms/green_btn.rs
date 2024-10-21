

#[derive(Properties, PartialEq)]
pub struct 3D_btn {
  #[prop_or_default]
  pub class: String,
}

#[function_component(3D_btn)]
pub fn dance_o_matic_logo(props: &DanceOMaticLogoProps) -> Html {
    let svg_ref = use_node_ref();