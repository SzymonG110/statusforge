use supabase::Client;

use super::config::Config;

pub fn create_client(config: &Config) -> Result<Client, supabase::Error> {
    Client::new(&config.supabase_url, &config.supabase_publishable_key)
}

pub fn create_client_with_secret(config: &Config) -> Result<Client, supabase::Error> {
    let secret = config
        .supabase_secret_key
        .as_deref()
        .ok_or_else(|| supabase::Error::config("SUPABASE_SECRET_KEY required"))?;
    Client::new_with_service_role(
        &config.supabase_url,
        &config.supabase_publishable_key,
        secret,
    )
}
