use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    update,
};

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

type IdStore = BTreeMap<String, BTreeSet<Principal>>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub subject: String,
    pub hash: String,
    pub book_name: String,
}   

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[update]
fn update(hash: String, name: String, book_name: String, subject: String, description: String) -> String {
    let principal = ic_cdk::api::caller();
    let new_profile = Profile {
        hash: hash.clone(),
        name: name.clone(),
        book_name: book_name.clone(),
        description: description.clone(),
        subject: subject.clone(),
    };

    PROFILE_STORE.with(|store| {
        let mut store = store.borrow_mut();

        let is_hash_exist = ID_STORE.with(|id_store| {
            id_store.borrow().get(&hash).map_or(false, |set| set.contains(&principal))
        });

        if is_hash_exist {
            let err = format!("This book already exists in our database.");
            String::from(err)
        } else {
            ID_STORE.with(|id_store| {
                let mut id_store = id_store.borrow_mut();
                id_store
                    .entry(hash.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(principal);
            });

            store.insert(principal, new_profile.clone());
            String::from("Your book is uploaded")
        }
    })
}
