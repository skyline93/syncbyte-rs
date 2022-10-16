use std::path::Path;
use std::str::FromStr;

use aws_config::provider_config::ProviderConfig;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{config::Builder, Client, Endpoint as SdkEndpoint};
use aws_types::os_shim_internal::Env;
use http::Uri;
use tokio::runtime::Runtime;

pub enum S3Error {
    PutError,
}

pub struct Options<'a> {
    pub endpoint: &'a str,
    pub access_key: &'a str,
    pub secret_key: &'a str,
    pub region: &'a str,
}

pub struct S3<'a> {
    client: Client,
    pub bucket: &'a str,
    rt: Runtime,
}

impl<'a> S3<'a> {
    pub fn new(rt: Runtime, options: &Options, bucket: &'a str) -> Self {
        let sdk_config = rt.block_on(
            aws_config::from_env()
                .configure(ProviderConfig::default().with_env(Env::from_slice(&[
                    ("AWS_ACCESS_KEY_ID", options.access_key),
                    ("AWS_SECRET_ACCESS_KEY", options.secret_key),
                    ("AWS_REGION", options.region),
                ])))
                .load(),
        );

        let s3_config = Builder::from(&sdk_config)
            .endpoint_resolver(SdkEndpoint::immutable(
                Uri::from_str(options.endpoint).unwrap(),
            ))
            .build();

        S3 {
            client: Client::from_conf(s3_config),
            bucket: bucket,
            rt: rt,
        }
    }

    async fn put_object(&self, filename: &str) -> Result<String, crate::s3::S3Error> {
        let result = ByteStream::from_path(Path::new(filename)).await;
        let body = match result {
            Ok(bs) => bs,
            Err(_) => return Err(crate::s3::S3Error::PutError),
        };

        let result = self
            .client
            .put_object()
            .bucket(self.bucket)
            .key(filename)
            .body(body)
            .send()
            .await;

        match result {
            Ok(msg) => Ok(format!("put object successful, message: {:?}", msg)),
            Err(_) => Err(crate::s3::S3Error::PutError),
        }
    }

    pub fn put(&self, filename: &str) -> Result<String, crate::s3::S3Error> {
        self.rt.block_on(self.put_object(filename))
    }
}
