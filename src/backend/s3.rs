use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::backend::{Error, S3Error};
use aws_config::provider_config::ProviderConfig;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{config::Builder, Client, Endpoint as SdkEndpoint};
use aws_types::os_shim_internal::Env;
use http::Uri;
use tokio::runtime::Runtime;

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
    pub fn new(options: &Options, bucket: &'a str) -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

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
        let metadata = fs::metadata(filename).unwrap();
        let size = metadata.len();

        let body = match ByteStream::from_path(Path::new(filename)).await {
            Ok(body) => body,
            Err(e) => {
                return Err(S3Error::PathError(Error {
                    message: e.to_string(),
                }))
            }
        };

        let output = match self
            .client
            .put_object()
            .bucket(self.bucket)
            .key(filename)
            .body(body)
            .send()
            .await
        {
            Ok(output) => output,
            Err(e) => {
                return Err(S3Error::PutError(Error {
                    message: e.to_string(),
                }))
            }
        };

        Ok(format!(
            "put object successful, size: {}, etag: {}",
            size,
            output.e_tag.unwrap()
        ))
    }

    pub fn put(&self, filename: &str) -> Result<String, crate::s3::S3Error> {
        self.rt.block_on(self.put_object(filename))
    }
}
