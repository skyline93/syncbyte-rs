use crate::source::{Error, SourceError};
use std::process::{Command, ExitStatus};

pub struct Options<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub db_name: &'a str,
}

impl<'a> Options<'a> {
    fn to_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

pub struct Postgres<'a> {
    options: &'a Options<'a>,
    // version: &'a str,
}

impl<'a> Postgres<'a> {
    pub fn new(opts: &'a Options<'a>) -> Self {
        Postgres {
            options: opts,
            // version: version,
        }
    }

    pub fn dump(&self, dest_file: &str) -> Result<(), SourceError> {
        let result = Command::new("pg_dump")
            .args([self.options.to_uri().as_str(), "-Fc", "-f", dest_file])
            .output();

        let output = match result {
            Ok(output) => output,
            Err(e) => {
                return Err(SourceError::CommandError(Error {
                    message: e.to_string(),
                }))
            }
        };

        if let Some(0) = ExitStatus::code(&output.status) {
            return Ok(());
        };

        Err(SourceError::DumpError(Error {
            message: String::from_utf8(output.stderr).unwrap(),
        }))
    }
}
