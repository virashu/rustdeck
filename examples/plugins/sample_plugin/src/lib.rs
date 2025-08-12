#![allow(clippy::unnecessary_wraps)]
#![allow(unsafe_op_in_unsafe_fn)]

use rustdeck_common::{
    Args, Type,
    builder::{Action, PluginBuilder, Variable},
    decorate_fn_get_variable, decorate_fn_init, decorate_fn_run_action, decorate_fn_update,
    export_plugin,
};

struct PluginState {
    counter: i32,
}

const fn init() -> Result<PluginState, String> {
    Ok(PluginState { counter: 0 })
}

const fn update(_: &PluginState) {}

fn get_variable(state: &PluginState, id: &str) -> Result<String, String> {
    Ok(if id == "counter" {
        state.counter.to_string()
    } else {
        unreachable!()
    })
}

fn run_action(
    state: &mut PluginState,
    id: &str,
    args: &Args,
) -> Result<(), Box<dyn std::error::Error>> {
    match id {
        "add" => {
            let amt = args.get(0).int();
            state.counter += amt;
        }
        "increment" => {
            state.counter += 1;
        }
        "clear" => {
            state.counter = 0;
        }
        _ => unreachable!(),
    }

    Ok(())
}

export_plugin! {
    PluginBuilder::new("plugin_test", "Sample Plugin", "A sample plugin")
        .init(decorate_fn_init!(init))
        .update(decorate_fn_update!(update))
        .get_variable(decorate_fn_get_variable!(get_variable))
        .run_action(decorate_fn_run_action!(run_action))

        /* Variables */
        .variable(Variable::new("counter", "Counter", Type::String))

        /* Actions */
        .action(Action::new("increment", "Increment", "Increment counter"))
        .action(
            Action::new("add", "Add", "Add value to counter")
                .arg("amount", "Amount", "Amount", Type::Int)
        )
        .action(Action::new("clear", "Clear", "Set counter value to 0"))

        .build()
        .unwrap()
}
