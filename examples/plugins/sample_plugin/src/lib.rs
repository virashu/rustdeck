use rustdeck_common::{
    Plugin, actions, decl_action, decl_plugin, decl_variable, export_plugin, variables,
};

struct PluginState {
    counter: i32,
}

const fn init() -> PluginState {
    PluginState { counter: 0 }
}

const fn update(_: &PluginState) {}

fn run_action(state: &mut PluginState, id: &str) {
    match id {
        "increment" => {
            state.counter += 1;
        }
        "clear" => {
            state.counter = 0;
        }
        _ => {}
    }
}

fn get_variable(state: &PluginState, id: &str) -> String {
    if id == "counter" {
        state.counter.to_string()
    } else {
        String::new()
    }
}

export_plugin! {
    decl_plugin! {
        id: "plugin_test",
        name: "Sample Plugin",
        desc: "A sample plugin",
        variables: variables!(
            decl_variable! {
                id: "counter",
                desc: "Counter",
                vtype: "string",
            },
        ),
        actions: actions!(
            decl_action! {
                id: "increment",
                name: "Increment",
                desc: "Increment counter"
            },
            decl_action! {
                id: "clear",
                name: "Clear",
                desc: "Set counter value to 0"
            },
        ),

        fn_init: init,
        fn_update: update,
        fn_get_variable: get_variable,
        fn_run_action: run_action,
    }
}
