use bytes::Bytes;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};
use std::str::FromStr;

use bamboo_common_core::error::{BambooError, BambooErrorResult, BambooResult};

#[derive(Clone)]
pub struct MinioClient {
    bucket: Bucket,
}

impl MinioClient {
    pub fn new(
        bucket_name: String,
        access_key: String,
        secret_key: String,
        region: String,
        endpoint: Option<String>,
        use_path_style: bool,
    ) -> Result<MinioClient, S3Error> {
        let region = if let Some(endpoint) = endpoint {
            Region::Custom { region, endpoint }
        } else {
            Region::from_str(region.as_str()).unwrap()
        };
        let credentials = Credentials::new(
            Some(access_key.as_str()),
            Some(secret_key.as_str()),
            None,
            None,
            None,
        )
        .map_err(|err| S3Error::Credentials(err))?;
        let mut bucket = Bucket::new(bucket_name.as_str(), region, credentials)?.with_path_style();
        if use_path_style {
            bucket.set_path_style();
        }

        Ok(MinioClient { bucket })
    }

    fn get_profile_picture_path(&self, user_id: i32) -> String {
        format!("/user/profile_picture/{user_id}")
    }

    pub async fn upload_profile_picture(&self, user_id: i32, data: &[u8]) -> BambooErrorResult {
        let response = self
            .bucket
            .put_object(self.get_profile_picture_path(user_id), data)
            .await
            .map_err(|err| {
                log::error!("Failed to save profile picture {err}");
                BambooError::io("user", "Failed to save profile picture")
            })?;
        if response.status_code() != 200 {
            Err(BambooError::io("user", "Failed to save profile picture"))
        } else {
            Ok(())
        }
    }

    pub async fn get_profile_picture(&self, user_id: i32) -> BambooResult<Bytes> {
        let response = self
            .bucket
            .get_object(self.get_profile_picture_path(user_id))
            .await
            .map_err(|err| {
                log::error!("Failed to get profile picture {err}");
                BambooError::io("user", "Failed to get profile picture")
            })?;
        if response.status_code() != 200 {
            Err(BambooError::io("user", "Failed to get profile picture"))
        } else {
            let data = response.bytes().clone();
            Ok(data)
        }
    }
}
