#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub supabase_url: String,
    pub supabase_publishable_key: String,
    pub supabase_secret_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::prefixed("")
            .from_env::<ConfigEnv>()
            .map(Config::from)
    }
}

#[derive(serde::Deserialize, Debug)]
struct ConfigEnv {
    port: Option<u16>,
    supabase_url: String,
    supabase_publishable_key: Option<String>,
    supabase_anon_key: Option<String>,
    supabase_secret_key: Option<String>,
}

impl From<ConfigEnv> for Config {
    fn from(e: ConfigEnv) -> Self {
        let supabase_publishable_key = e
            .supabase_publishable_key
            .or(e.supabase_anon_key)
            .expect("Missing env: SUPABASE_PUBLISHABLE_KEY or SUPABASE_ANON_KEY");
        Config {
            port: e.port.unwrap_or(3001),
            supabase_url: e.supabase_url,
            supabase_publishable_key,
            supabase_secret_key: e.supabase_secret_key,
        }
    }
}
