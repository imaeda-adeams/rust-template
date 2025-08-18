use std::env;
use strum::EnumString;

#[derive(Default, EnumString, Debug, PartialEq, Eq)]
pub enum Environment {
    // 開発環境向けで動作していることを示す
    #[default]
    Development,
    // 本番環境向けで動作していることを示す
    Production,
}

/// 開発環境か本番環境かを判定する
/// 判定基準は以下とする。
///   * 環境変数 ENV で、
///     - production: と指定されていれば、Environment::Production を返し
///     - Development: と指定されていれば、Environment::Development を返す。
///   * 環境変数 ENV が設定されていない、または、不正の値が設定されていたら
///     - デバッグビルドであれば、Environment::Development を返し、
///     - リリースビルドであれば、Environment::Production を返す。
///
///  # Returns
///    環境を示す Enum
pub fn which() -> Environment {

    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}

#[test]
pub fn test_which_1() {

    unsafe {
        std::env::set_var("ENV", "Production");

        let env = which();
        assert_eq!(env, Environment::Production);
    }
}

#[test]
pub fn test_which_2() {

    unsafe {
        std::env::set_var("ENV", "Development");

        let env = which();
        assert_eq!(env, Environment::Development)
    }
}

#[test]
pub fn test_which_3() {

    let env = which();
    assert_eq!(env, Environment::Development)
}
