use rusoto_core::credential::AwsCredentials;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};

pub fn put_object(bucket: String, key: String, data: Vec<u8>) {
    let client = S3Client::new(get_region());

    let body = Some(ByteStream::from(data));
    let content_type = Some("video/mp4".to_owned());

    let output = client
        .put_object(PutObjectRequest {
            bucket,
            key,
            body,
            content_type,
            ..Default::default()
        })
        .sync()
        .unwrap();

    println!("{:?}", output);
}

pub fn get_presigned_url(bucket: String, key: String) -> String {
    GetObjectRequest {
        bucket,
        key,
        ..Default::default()
    }
    .get_presigned_url(
        &get_region(),
        &get_credentials(),
        &PreSignedRequestOption {
            expires_in: ::std::time::Duration::from_secs(3600),
        },
    )
}

fn get_region() -> Region {
    Region::Custom {
        name: "minio".to_owned(),
        endpoint: "http://localhost:9001".to_owned(),
    }
}

fn get_credentials() -> AwsCredentials {
    AwsCredentials::new("minio", "minio123", None, None)
}
