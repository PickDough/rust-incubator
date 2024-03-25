use std::{borrow::Cow, env};

const DEFAULT_CONF_PATH: &str = "/etc/app/app.conf";

fn extract_conf_path<V, A>(mut vars: V, mut args: A) -> Cow<'static, str>
where
    V: Iterator<Item = (String, String)>,
    A: Iterator<Item = String>,
{
    let env_var = vars.find(|e| e.0 == "APP_CONF").map(|e| e.1);

    let env_arg_pos = args.position(|a| a == "--conf");
    let env_arg = env_arg_pos.and_then(|_| args.next());

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

fn main() {
    let conf = extract_conf_path(env::vars(), env::args());

    println!("{}", conf);
}
#[cfg(test)]
mod tests {
    use std::env;

    use crate::{extract_conf_path, DEFAULT_CONF_PATH};

    #[test]
    fn test_default() {
        assert_eq!(
            extract_conf_path(vec![].into_iter(), vec![].into_iter()),
            DEFAULT_CONF_PATH
        );
    }

    #[test]
    fn test_var() {
        let path = "env/var/app.conf";
        env::set_var("APP_CONF", path);

        assert_eq!(extract_conf_path(env::vars(), env::args()), path);
    }

    #[test]
    #[should_panic(expected = "--conf can't be empty")]
    fn test_panic() {
        let args = vec!["--conf".to_owned()];

        extract_conf_path(env::vars(), args.into_iter());
    }

    #[test]
    fn test_arg() {
        let path_var = "env/var/app.conf";
        let path_arg = "env/arg/app.conf";
        env::set_var("APP_CONF", path_var);

        let args = vec!["--conf".to_owned(), path_arg.to_owned()];

        assert_eq!(extract_conf_path(env::vars(), args.into_iter()), path_arg);
    }
}
