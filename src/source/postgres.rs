use std::io::{Error, ErrorKind};
use std::process::Output;
use std::{os::unix::process::CommandExt, process::Command};

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
    version: &'a str,
}

impl<'a> Postgres<'a> {
    pub fn new(opts: &'a Options<'a>, version: &'a str) -> Self {
        Postgres {
            options: opts,
            version: version,
        }
    }

    pub fn dump(&self, dest_file: &str) -> Result<Output, Error> {
        let err = Command::new("pg_dump")
            .args([self.options.to_uri().as_str(), "-Fc", "-f", dest_file])
            .output();

        err
    }
}
