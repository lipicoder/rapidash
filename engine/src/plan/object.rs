//! Feature object store provider.
use datafusion::datasource::object_store::ObjectStoreProvider;
use datafusion::error::DataFusionError;
use object_store::ObjectStore;
use std::sync::Arc;
use url::Url;

/// An object store detector based on which features are enable for different kinds of object stores
pub struct FeatureBasedObjectStoreProvider;

impl ObjectStoreProvider for FeatureBasedObjectStoreProvider {
    /// Detector a suitable object store based on its url if possible
    /// Return the key and object store
    #[allow(unused_variables)]
    fn get_by_url(&self, url: &Url) -> datafusion::error::Result<Arc<dyn ObjectStore>> {
        #[cfg(any(feature = "hdfs", feature = "hdfs3"))]
        {
            let store = HadoopFileSystem::new(url.as_str());
            if let Some(store) = store {
                return Ok(Arc::new(store));
            }
        }

        #[cfg(feature = "s3")]
        {
            if url.to_string().starts_with("s3://") {
                if let Some(bucket_name) = url.host_str() {
                    let store = AmazonS3Builder::from_env()
                        .with_bucket_name(bucket_name)
                        .build()?;
                    return Ok(Arc::new(store));
                }
            }
        }

        Err(DataFusionError::Execution(format!(
            "No object store available for {}",
            url
        )))
    }
}
