use futures::executor::block_on;
use media_session::{MediaSession, traits::MediaSessionControls};
use rustdeck_common::{Plugin, actions, decl_action, decl_plugin, decl_variable, variables};

use std::{
    panic::catch_unwind,
};

// fn string_to_ptr(s: String) -> *mut c_char {
//     let value = ManuallyDrop::new(Box::new(CString::new(s).unwrap()));

//     (*value).as_ptr().cast_mut()
// }

struct PluginState {
    player: MediaSession,
}

fn init() -> PluginState {
    PluginState {
        player: block_on(MediaSession::new()),
    }
}

fn update(_: &mut PluginState) {}

fn run_action(state: &PluginState, id: &str) {
    if id == "play_pause" {
        block_on(async { state.player.toggle_pause().await.unwrap() });
    }
}

fn get_variable(_: &PluginState, id: &str) -> String {
    let Ok(media_info) = catch_unwind(|| block_on(MediaSession::new()).get_info()) else {
        return String::new();
    };

    match id {
        "title" => media_info.title,
        "artist" => media_info.artist,
        "state" => media_info.state,
        _ => String::new(),
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn build() -> *const Plugin {
    decl_plugin! {
        id: "rustdeck_media",
        name: "RustDeck Media Plugin",
        desc: "A plugin for media management (music, video, etc.)",
        variables: variables!(
            decl_variable! {
                id: "title",
                desc: "Title",
                vtype: "string",
            },
            decl_variable! {
                id: "artist",
                desc: "Artist",
                vtype: "string",
            },
            decl_variable! {
                id: "state",
                desc: "State",
                vtype: "string",
            },
        ),
        actions: actions!(
            decl_action! {
                id: "play_pause",
                name: "Pause toggle",
                desc: "Toggle play/pause"
            },
        ),

        fn_init: init,
        fn_update: update,
        fn_get_variable: get_variable,
        fn_run_action: run_action,
    }
}
