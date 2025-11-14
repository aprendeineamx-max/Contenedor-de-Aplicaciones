use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use orbit_cli_sdk::apis::{configuration::Configuration, system_api};

#[derive(Parser, Debug)]
#[command(name = "orbit", version, about = "CLI para el agente Orbit")]
struct OrbitCli {
    /// URL base del agente (env: ORBIT_BASE_URL)
    #[arg(long, env = "ORBIT_BASE_URL", default_value = "http://127.0.0.1:7443")]
    base_url: String,

    /// Token Bearer con permisos admin (env: ORBIT_ADMIN_TOKEN)
    #[arg(long, env = "ORBIT_ADMIN_TOKEN")]
    admin_token: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Mostrar snapshot de configuracion (equivalente a GET /system/config)
    SystemConfig {
        /// Formato de salida
        #[arg(long, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = OrbitCli::parse();
    match cli.command {
        Commands::SystemConfig { format } => {
            system_config(cli.base_url, cli.admin_token, format).await?
        }
    }
    Ok(())
}

async fn system_config(
    base_url: String,
    admin_token: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    let token = admin_token
        .or_else(|| std::env::var("ORBIT_ADMIN_TOKEN").ok())
        .context("Debes proporcionar --admin-token o la variable ORBIT_ADMIN_TOKEN")?;

    let mut configuration = Configuration::new();
    configuration.base_path = base_url;
    configuration.bearer_access_token = Some(token);

    let snapshot = system_api::system_config_get(&configuration)
        .await
        .context("No se pudo consultar /system/config")?;

    match format {
        OutputFormat::Pretty => {
            println!(
                "{}",
                serde_json::to_string_pretty(&snapshot)
                    .context("No se pudo serializar el snapshot")?
            );
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string(&snapshot).context("No se pudo serializar el snapshot")?
            );
        }
    }

    Ok(())
}
