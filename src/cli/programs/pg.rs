use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Pg {
    #[command(subcommand)]
    pub command: PgCommand
}

#[derive(Debug, Subcommand)]
pub enum PgCommand {
    /// Creates test and development databases without running migrations.
    Bootstrap(bootstrap::BootstrapArgs),

    /// Migration utilities.
    Migrate(migrate::MigrateArgs)
}

pub mod bootstrap {
    use clap::Args;
    use crate::metadata::{DATABASE_DEV, DATABASE_TEST, DATABASE_OWNER};
    use std::io;
    use std::process::Command;
    use super::super::super::utils::print_status;

    #[derive(Args, Debug)]
    pub struct BootstrapArgs {
        /// Drop databases and bootstrap
        #[arg(short, long)]
        redo: bool
    }

    pub fn bootstrap(BootstrapArgs { redo }: BootstrapArgs) -> io::Result<()> {
        if redo {
            reinitialize()
        } else {
            initialize()
        }
    }

    fn reinitialize() -> io::Result<()> {
        Command::new("psql")
            .args(["template1", "-c", &format!("DROP DATABASE {DATABASE_DEV}")])
            .output()
            .map(print_status)?;

        Command::new("psql")
            .args(["template1", "-c", &format!("DROP DATABASE {DATABASE_TEST}")])
            .output()
            .map(print_status)?;

        initialize()?;

        Ok(())
    }

    fn initialize() -> io::Result<()> {
        Command::new("psql")
            .args(["template1", "-c", &format!("CREATE USER {DATABASE_OWNER}")])
            .output()
            .map(print_status)?;

        Command::new("psql")
            .args(["template1", "-c", &format!("CREATE DATABASE {DATABASE_DEV} WITH OWNER = '{DATABASE_OWNER}'")])
            .output()
            .map(print_status)?;

        Command::new("psql")
            .args(["template1", "-c", &format!("CREATE DATABASE {DATABASE_TEST} WITH OWNER = '{DATABASE_OWNER}'")])
            .output()
            .map(print_status)?;

        Ok(())
    }
}

pub mod migrate {
    use clap::Args;
    use std::io;
    use std::process::Command;
    use super::super::super::utils::print_status;

    #[derive(Args, Debug)]
    pub struct MigrateArgs {
        /// [run | revert]
        mode: String,

        /// Which database to run migrations on. Defaults to dev. [dev | test] 
        #[arg(short, long)]
        environment: Option<String>,
    }

    pub fn migrate(args: MigrateArgs) -> io::Result<()> {
        assert!(
            &args.mode == "run" || &args.mode == "revert",
            "Invalid argument provided for 'mode'"
        );

        let db_uri = {
            let uri = if let Some(ref env) = args.environment {
                if env == "test" {
                    dotenv::var("DATABASE_URL_TEST").ok()
                } else {
                    None
                }
            } else {
                dotenv::var("DATABASE_URL").ok()
            };

            uri.expect("Failed to find 'DATABASE_URL_TEST' in .env file")
        };

        Command::new("sqlx")
            .args(["migrate", &args.mode, "-D", &db_uri])
            .output()
            .map(print_status)?;

        Ok(())
    }
}
