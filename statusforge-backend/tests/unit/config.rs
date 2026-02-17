use statusforge_backend::shared::config::Config;
use statusforge_backend::shared::config::ConfigEnv;

#[test]
fn test_config_from_env_default_port() {
    let env = ConfigEnv {
        port: None,
        supabase_url: "https://test.supabase.co".to_string(),
        supabase_publishable_key: Some("test-key".to_string()),
        supabase_anon_key: None,
        supabase_secret_key: None,
    };
    let config = Config::from(env);
    assert_eq!(config.port, 3001);
}

#[test]
fn test_config_from_env_custom_port() {
    let env = ConfigEnv {
        port: Some(8080),
        supabase_url: "https://test.supabase.co".to_string(),
        supabase_publishable_key: Some("test-key".to_string()),
        supabase_anon_key: None,
        supabase_secret_key: None,
    };
    let config = Config::from(env);
    assert_eq!(config.port, 8080);
}

#[test]
fn test_config_prefers_publishable_key_over_anon_key() {
    let env = ConfigEnv {
        port: None,
        supabase_url: "https://test.supabase.co".to_string(),
        supabase_publishable_key: Some("publishable-key".to_string()),
        supabase_anon_key: Some("anon-key".to_string()),
        supabase_secret_key: None,
    };
    let config = Config::from(env);
    assert_eq!(config.supabase_publishable_key, "publishable-key");
}

#[test]
fn test_config_falls_back_to_anon_key() {
    let env = ConfigEnv {
        port: None,
        supabase_url: "https://test.supabase.co".to_string(),
        supabase_publishable_key: None,
        supabase_anon_key: Some("anon-key".to_string()),
        supabase_secret_key: None,
    };
    let config = Config::from(env);
    assert_eq!(config.supabase_publishable_key, "anon-key");
}
