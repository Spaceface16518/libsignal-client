use crate::error::Result;
use crate::state::{PreKeyId, PreKeyRecord, SessionRecord, SignedPreKeyId, SignedPreKeyRecord};
use crate::{IdentityKey, IdentityKeyPair, ProtocolAddress};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    Sending,
    Receiving,
}

pub trait IdentityKeyStore {
    fn get_identity_key_pair(&self) -> Result<IdentityKeyPair>;

    fn get_local_registration_id(&self) -> Result<u32>;

    fn save_identity(&mut self, address: &ProtocolAddress, identity: &IdentityKey) -> Result<bool>;

    fn is_trusted_identity(
        &self,
        address: &ProtocolAddress,
        identity: &IdentityKey,
        direction: Direction,
    ) -> Result<bool>;

    fn get_identity(&self, address: &ProtocolAddress) -> Result<Option<IdentityKey>>;
}

pub trait PreKeyStore {
    fn get_pre_key(&self, prekey_id: PreKeyId) -> Result<PreKeyRecord>;

    fn save_pre_key(&mut self, prekey_id: PreKeyId, record: &PreKeyRecord) -> Result<()>;

    fn has_pre_key(&self, prekey_id: PreKeyId) -> Result<bool>;

    fn remove_pre_key(&mut self, prekey_id: PreKeyId) -> Result<()>;
}

pub trait SignedPreKeyStore {
    fn get_signed_pre_key(&self, signed_prekey_id: SignedPreKeyId) -> Result<SignedPreKeyRecord>;

    fn get_all_signed_prekeys(&self) -> Result<Vec<SignedPreKeyRecord>>;

    fn save_signed_pre_key(
        &mut self,
        signed_prekey_id: SignedPreKeyId,
        record: &SignedPreKeyRecord,
    ) -> Result<()>;

    fn has_signed_pre_key(&self, signed_prekey_id: SignedPreKeyId) -> Result<bool>;

    fn remove_pre_key(&mut self, signed_prekey_id: SignedPreKeyId) -> Result<()>;
}

pub trait SessionStore {
    fn load_session(&self, address: &ProtocolAddress) -> Result<Option<SessionRecord>>;

    fn get_sub_device_sessions(&self, name: &str) -> Result<Vec<u32>>;

    fn store_session(&mut self, address: &ProtocolAddress, record: &SessionRecord) -> Result<()>;

    fn contains_session(&self, address: &ProtocolAddress) -> Result<bool>;

    fn delete_session(&mut self, address: &ProtocolAddress) -> Result<()>;

    fn delete_all_sessions(&mut self, name: &str) -> Result<()>;
}