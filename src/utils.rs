use futures::Future;

use crate::enums::ImageFormat;
use std::path::Path;

pub fn get_extension(path: &str) -> Option<&str> {
    match Path::new(path).extension() {
        Some(ext) => ext.to_str(),
        None => None,
    }
}

pub fn parse_path(path: &str) -> (Option<ImageFormat>, Option<&str>) {
    let extension = get_extension(path);
    (ImageFormat::from(extension.unwrap_or("")), extension)
}

pub async fn join_parallel<T: Send + 'static>(
    futs: impl IntoIterator<Item = impl Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let tasks: Vec<_> = futs.into_iter().map(tokio::spawn).collect();
    // unwrap the Result because it is introduced by tokio::spawn()
    // and isn't something our caller can handle
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .collect()
}
