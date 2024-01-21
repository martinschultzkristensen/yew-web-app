use crate::components::molecules::video_list::Video;

use std::rc::Rc;
use core::cell::RefCell;

let intro_video = get_intro_video();
    let play_intro = true;
    let play_intro = play_intro;
    let play_intro_ref = Rc::new(RefCell::new(play_intro));


    let handle_keydown = {
        let intro_video = intro_video.clone();
        let videos = videos.clone();
        let play_intro = Rc::clone(&play_intro_ref);
        
        let callback = Callback::from(move |event: KeyboardEvent| {
            if event.key() == "x" {
                let mut play_intro = play_intro.borrow_mut();
                if *play_intro {
                    // Play intro video logic...
                    *play_intro = false;
                } else {
                    // Trigger demo videos logic...
                    let _ = get_demo_videos(); // Call the function
                }
            }
        });
    
        callback
    };

    html! {
    <div onkeydown={handle_keydown} tabindex="0">
        <VideosList videos={intro_video} current_index={0} />
    </div>
    }