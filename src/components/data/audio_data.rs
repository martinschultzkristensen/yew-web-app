use crate::components::molecules::audio_list::Audio;



pub fn get_menu_audio() -> Vec<Audio> {
    vec![
        Audio {
            id: 0,
            url: "static/8bit-menusong-short-ed.aif".to_string(),
        },
    ]
}
