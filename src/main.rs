pub mod backend;
pub mod source;

use crate::backend::s3;
use crate::source::postgres;
use std::process::exit;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let opts = postgres::Options {
        host: "127.0.0.1",
        port: 5432,
        user: "syncbyte",
        password: "123456",
        db_name: "syncbyte",
    };

    let pg = postgres::Postgres::new(&opts, "14.5");
    let err = pg.dump("core_cms");

    match err {
        Ok(_) => (),
        Err(msg) => {
            println!("error: {}", msg);
            exit(1)
        }
    }

    let s3_opts = s3::Options {
        endpoint: "http://127.0.0.1:9000",
        access_key: "accesskey123",
        secret_key: "secretkey123",
        region: "",
    };

    let bak = s3::S3::new(rt, &s3_opts, "syncbyte");
    match bak.put("core_cms") {
        Ok(msg) => {
            println!("{}", msg)
        }
        Err(_) => exit(1),
    };
}
