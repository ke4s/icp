use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub subject: String,
    pub hash: String,
    pub bookName: String,
}   

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[query(name = "getSelf")]
fn get_self() -> Profile {
    let id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .cloned().unwrap_or_default()
    })
}

fn get(hash: String) -> Option<Profile> {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&hash)
                .and_then(|id| profile_store.borrow().get(id).cloned())
        })
    })
}

#[update]
fn update(new_profile: Profile) -> Option<Profile> {
    match get(new_profile.hash.clone()) {
        Some(profile) => {
            // Profil zaten var, istediğiniz işlemleri yapabilirsiniz.
            // Ancak, dönüş türü Option<Profile> olduğu için burada bir değer döndürmek zorundasınız.
            // Eğer bir değer döndürmek istemiyorsanız, bu satırı kaldırabilirsiniz.
            Some(profile)
        }
        None => {
            let principal_id = ic_cdk::api::caller();
            ID_STORE.with(|id_store| {
                id_store
                    .borrow_mut()
                    .insert(new_profile.name.clone(), principal_id);
            });
            PROFILE_STORE.with(|profile_store| {
                profile_store
                    .borrow_mut()
                    .insert(principal_id, new_profile.clone());
            });

            // Eğer bir değer döndürmek istemiyorsanız, sadece None döndürün.
            None
        }
    }
}

/*

fn update(new_profile: Profile) -> Option<Profile> {

    match get(new_profile.hash) {
        Some(profile) => {
            return profile;
        }
        None => {
            let principal_id = ic_cdk::api::caller();
            ID_STORE.with(|id_store| {
                id_store
                    .borrow_mut()
                    .insert(new_profile.name.clone(), principal_id);
            });
            PROFILE_STORE.with(|profile_store| {
                profile_store.borrow_mut().insert(principal_id, new_profile.clone());
            });
            None
        }
    }


}
*/