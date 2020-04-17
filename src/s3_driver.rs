use s3::bucket::Bucket;
use s3::region::{Region};
use s3::credentials::Credentials;
use s3::error::S3Error;

const BUCKET_NAME: &str = "watplan";

const REGION: &str = "eu-central-1";



pub fn get_bucket() -> Result<Bucket,S3Error> {
    let access_key: String = std::env::var("AWSAccessKeyId").expect("AWSAccessKeyId not set");
    let secret_key: String = std::env::var("AWSSecretKey").expect("AWSSecretKey not set");
    let credentials = Credentials::new_blocking(Some(access_key), Some(secret_key), None, None).unwrap();
    let region: Region = REGION.parse().unwrap();

    Bucket::new(BUCKET_NAME,region,credentials)
}
