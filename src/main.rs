#![deny(warnings, unused_crate_dependencies)]

use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use app::{
    app_start,
    entity::{DatabaseConfig, db_connect},
    web_state::WebState,
};
use clap::Parser;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(env, short, long, default_value = "0.0.0.0:8085")]
    pub listen: String,

    #[clap(env, short = 'g', long)]
    pub log_dir: Option<PathBuf>,

    #[clap(
        env,
        short,
        long,
        default_value = "postgres://postgres:123456@127.0.0.1:5432/postgres"
    )]
    pub db_url: String,
}

impl Args {
    pub async fn execute(self) -> Result<()> {
        let log_builder = tracing_subscriber::registry().with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        );

        if let Some(log_dir) = &self.log_dir {
            let file_appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix(current_exe())
                .filename_suffix("log")
                .build(log_dir)?;

            log_builder
                .with(tracing_subscriber::fmt::layer().with_writer(file_appender))
                .init();
        } else {
            log_builder.with(tracing_subscriber::fmt::layer()).init();
        };

        tracing::trace!(
            "Starting server listen:{} log dir: {:?}, db url: {:?}",
            self.listen,
            self.log_dir,
            self.db_url
        );

        let db = db_connect(&DatabaseConfig {
            url: self.db_url,
            enable_logging: true,
            ..Default::default()
        })
        .await?;
        let state = Arc::new(WebState::new(db));

        let addr = self.listen.parse::<SocketAddr>()?;
        app_start(&addr, state).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        if env::var_os("RUST_LOG").is_none() {
            env::set_var("RUST_LOG", "info");
        }
        if env::var_os("RUST_BACKTRACE").is_none() {
            env::set_var("RUST_BACKTRACE", "full");
        }
    }
    let args = Args::parse();
    args.execute().await
}

fn current_exe() -> String {
    env::current_exe()
        .ok()
        .and_then(|exe| {
            exe.file_stem()
                .map(|file_name| file_name.to_string_lossy().to_string())
        })
        .unwrap_or("services".to_string())
}
