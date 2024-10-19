// src/components/atoms/dance_o_matic_logo.rs
use std::rc::Rc;
use std::cell::RefCell;
use gloo_timers::callback::Timeout;
use web_sys::{Element, HtmlElement};
use yew::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

#[derive(Properties, PartialEq)]
pub struct DanceOMaticLogoProps {}

#[function_component(DanceOMaticLogo)]
pub fn dance_o_matic_logo(_props: &DanceOMaticLogoProps) -> Html {
    let svg_ref = use_node_ref();

    let show_font_pink_letters = {
        let svg_ref = svg_ref.clone();
        Callback::from(move |_| {
            if let Some(svg) = svg_ref.cast::<Element>() {
                let letters = svg.query_selector_all(".letter").unwrap();
                let letters_count = letters.length();

                for index in 0..letters_count {
                    if let Some(letter) = letters.get(index) {
                        let letter = letter.dyn_into::<HtmlElement>().unwrap();
                        let delay = if index == letters_count - 11 {
                        150
                    } else {
                        110
                    };
                    let timeout = Timeout::new((letters_count - index) as u32 * delay, move || {
                        letter.class_list().add_1("show").unwrap();
                    });
                    timeout.forget();
                }
            }
        }
    })
};

    let fade_in_shadow_effect = {
        let svg_ref = svg_ref.clone();
        Callback::from(move |_| {
            if let Some(svg) = svg_ref.cast::<Element>() {
                if let Some(shadow_element) = svg.query_selector("#shadeEffect").unwrap() {
                    shadow_element.class_list().add_1("show").unwrap();
                }
            }
        })
    };

    let svg_ref_effect = svg_ref.clone();
    let show_font_pink_letters_effect = show_font_pink_letters.clone();
    let fade_in_shadow_effect_effect = Rc::new(RefCell::new(fade_in_shadow_effect.clone()));

    use_effect(move || {
        if let Some(svg) = svg_ref_effect.cast::<Element>() {
            if let Some(pil) = svg.query_selector("#Pil").unwrap() {
                let fade_effect = fade_in_shadow_effect_effect.clone();
                let on_animation_end = Closure::wrap(Box::new(move || {
                    show_font_pink_letters_effect.emit(());
                    let fade_effect_clone = fade_effect.clone();
                    let timeout = Timeout::new(2200, move || {
                        fade_effect_clone.borrow().emit(());
                    });
                    timeout.forget();
                }) as Box<dyn FnMut()>);

                pil.add_event_listener_with_callback(
                    "animationend",
                    on_animation_end.as_ref().unchecked_ref(),
                )
                .unwrap();
                on_animation_end.forget();
            }
        }
        || ()
    });

    html! {
        <div>
        
        </div>
    }
}
