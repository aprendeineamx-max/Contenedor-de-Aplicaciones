use orbit_cli_sdk::apis::{configuration::Configuration, system_api};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = env::var("ORBIT_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:7443".into());
    let token = env::var("ORBIT_ADMIN_TOKEN")
        .expect("Definir ORBIT_ADMIN_TOKEN para consultar /system/config");

    let mut configuration = Configuration::new();
    configuration.base_path = base;
    configuration.bearer_access_token = Some(token);

    let snapshot = system_api::system_config_get(&configuration).await?;
    println!("{}", serde_json::to_string_pretty(&snapshot)?);
    Ok(())
}
