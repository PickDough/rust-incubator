use std::{borrow::Cow, env};

const DEFAULT_CONF_PATH: &'static str = "/etc/app/app.conf";

fn extract_conf_path() -> Cow<'static, str> {
    let env_var = env::vars().find(|e| e.0 == "APP_CONF").map(|e| e.1);

    let env_arg_pos = env::args().into_iter().position(|a| a == "--conf");
    let env_arg = env_arg_pos.and(env::args().into_iter().skip(env_arg_pos.unwrap()).next());

    assert!(
        !(env_arg_pos.is_some() && env_arg.is_none()),
        "--conf can't be empty"
    );

    if let Some(s) = env_arg.or(env_var) {
        Cow::from(s)
    } else {
        Cow::from(DEFAULT_CONF_PATH)
    }
}

fn main() {}
