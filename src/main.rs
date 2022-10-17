pub mod backend;
pub mod source;

use crate::backend::s3;
use crate::source::postgres;
use std::process::exit;

fn main() {
    let opts = postgres::Options {
        host: "127.0.0.1",
        port: 5432,
        user: "syncbyte",
        password: "123456",
        db_name: "syncbyte",
    };

    let pg = postgres::Postgres::new(&opts);

    match pg.dump("core_cms1") {
        Ok(_) => (),
        Err(e) => {
            match e {
                source::SourceError::DumpError(e) => println!("{}", e),
                source::SourceError::CommandError(_) => (),
            };
            exit(1)
        }
    };

    let s3_opts = s3::Options {
        endpoint: "http://127.0.0.1:9000",
        access_key: "accesskey123",
        secret_key: "secretkey123",
        region: "",
    };

    match s3::S3::new(&s3_opts, "syncbyte").put("core_cms1") {
        Ok(msg) => {
            println!("{}", msg)
        }
        Err(e) => {
            match e {
                backend::S3Error::PathError(e) => println!("{}", e),
                backend::S3Error::PutError(e) => println!("{}", e),
            }
            exit(1)
        }
    };
}
