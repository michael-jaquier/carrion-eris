use dashmap::DashMap;
use tracing::error;

use crate::game::mutations::Mutations;


type MutationVector = Vec<Mutations>;

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub mutations: DashMap<u64, MutationVector>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            mutations: DashMap::new(),
        }
    }

    pub fn extend(&self, mutation: Vec<Mutations>) {
        let mut m = false;
        let mut retries = 0;
        let key = *mutation[0].user_id();
        while !m {
            match self.mutations.try_get(&key) {
                dashmap::try_result::TryResult::Present(r) => {
                    let mut mutations = r.value().clone();
                    mutations.extend(mutation.clone());
                    self.mutations.insert(key, mutation.clone());
                    m = true;
                }

                dashmap::try_result::TryResult::Absent => {
                    self.mutations.insert(key, mutation.clone());
                    m = true;
                }

                dashmap::try_result::TryResult::Locked => {
                    retries += 1;
                    if retries > 10 {
                        error!("Unable to get lock for mutations {:?}", mutation);
                        break;
                    }
                }
            }
        }
    }

    pub fn add(&self, mutation: Mutations) {
        let mut m = false;
        let mut retries = 0;
        let key = *mutation.user_id();
        let mut mutations_to_set = vec![mutation.clone()];
        while !m {
            match self.mutations.try_get(&key) {
                dashmap::try_result::TryResult::Present(r) => {
                    let mut mutations = r.value().clone();
                    mutations.push(mutation.clone());
                    mutations_to_set = mutations;
                    self.mutations.insert(key, mutations_to_set.clone());
                    m = true;
                }

                dashmap::try_result::TryResult::Absent => {
                    self.mutations.insert(key, mutations_to_set.clone());
                    m = true;
                }

                dashmap::try_result::TryResult::Locked => {
                    retries += 1;
                    if retries > 10 {
                        error!("Unable to get lock for mutations {:?}", mutation);
                        break;
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> dashmap::iter::Iter<'_, u64, Vec<Mutations>> {
        self.mutations.iter()
    }

    pub fn clear(&self, character: u64) {
        self.mutations.remove(&character);
    }

    pub fn get(&self, key: &u64) -> Option<dashmap::mapref::one::Ref<'_, u64, Vec<Mutations>>> {
        self.mutations.get(key)
    }
  
}
