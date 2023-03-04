use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use futures::{
    lock::{Mutex, OwnedMutexGuard},
    Future, TryFuture, TryFutureExt,
};

pub struct CachedMap<K, V>(Mutex<HashMap<K, Arc<Mutex<Option<V>>>>>);

impl<K, V> Default for CachedMap<K, V> {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl<K, V> CachedMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    async fn get_locked(&self, key: K) -> OwnedMutexGuard<Option<V>> {
        let mut m = self.0.lock().await;
        m.entry(key)
            .or_insert_with(|| Arc::new(Mutex::new(None)))
            .clone()
            .lock_owned() // lock is aquired before the whole map lock is released
            .await
    }

    #[allow(dead_code)]
    pub async fn get_or_insert_with<F, Fut, T>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
        T: Into<V>,
    {
        let mut v = self.get_locked(key).await;
        match &*v {
            Some(v) => v.clone(),
            None => v.insert(f().await.into()).clone(),
        }
    }

    #[allow(dead_code)]
    pub async fn get_or_try_insert_with<F, Fut, T>(&self, key: K, f: F) -> Result<V, Fut::Error>
    where
        F: FnOnce() -> Fut,
        Fut: TryFuture<Ok = T>,
        T: Into<V>,
    {
        let mut v = self.get_locked(key).await;
        Ok(match &*v {
            Some(v) => v.clone(),
            None => v.insert(f().into_future().await?.into()).clone(),
        })
    }
}
