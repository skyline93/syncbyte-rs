use std::io::Error;
use std::{os::unix::process::CommandExt, process::Command};

struct Options<'a> {
    host: &'a str,
    port: u16,
    user: &'a str,
    password: &'a str,
    db_name: &'a str,
}

impl<'a> Options<'a> {
    fn to_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

struct Postgres<'a> {
    options: &'a Options<'a>,
    version: &'a str,
}

impl<'a> Postgres<'a> {
    fn new(opts: &'a Options<'a>, version: &'a str) -> Self {
        Postgres {
            options: opts,
            version: version,
        }
    }

    fn dump(&self, dest_file: &str) -> Error {
        Command::new("pg_dump")
            .args([self.options.to_uri().as_str(), "-Fc", "-f", dest_file])
            .exec()
    }
}

fn main() {
    let opts = Options {
        host: "127.0.0.1",
        port: 5432,
        user: "syncbyte",
        password: "123456",
        db_name: "syncbyte",
    };

    let pg = Postgres::new(&opts, "14.5");
    let err = pg.dump("core_cms");

    println!("{}", err)
}
