use error::{Error, ErrorKind};
use failure::ResultExt;
use super::{Signature, Signer};

use yubihsm::{Algorithm, Capability, Domain, Session, Yubihsm};

const DEFAULT_CAPABILITIES: [Capability; 2] =
    [Capability::AsymmetricSignEddsa, Capability::ExportUnderWrap];

/// Key ID
pub type KeyID = u16;

/// Numerical Domain ID (1-16)
pub type DomainNumber = u8;

/// Create a new session with the YubiHSM2
pub fn session(connector_url: &str, auth_key_id: KeyID, password: &str) -> Result<Session, Error> {
    let hsm = Yubihsm::new().context(ErrorKind::ProviderError)?;
    let conn = hsm.create_connector(connector_url)
        .context(ErrorKind::ProviderError)?;

    conn.connect().context(ErrorKind::ProviderError)?;

    let session = conn.create_session_from_password(auth_key_id, password, true)
        .context(ErrorKind::ProviderError)?;

    Ok(session)
}

/// Generate a new Ed25519 key
pub fn generate_key(
    session: Session,
    signing_key_id: KeyID,
    label: &str,
    domain_number: DomainNumber,
) -> Result<(), Error> {
    let domain = Domain::new(domain_number).context(ErrorKind::ProviderError)?;

    session
        .generate_key_ed(
            signing_key_id,
            label,
            &[domain],
            &DEFAULT_CAPABILITIES,
            Algorithm::EcEd25519,
        )
        .context(ErrorKind::ProviderError)?;

    Ok(())
}

/// Ed25519 signature provider for yubihsm-rs
pub struct YubihsmSigner {
    session: Session,
    key_id: KeyID,
}

impl YubihsmSigner {
    /// Create a new YubihsmSigner
    pub fn new(
        connector_url: &str,
        auth_key_id: KeyID,
        signing_key_id: KeyID,
        password: &str,
    ) -> Result<Self, Error> {
        let session = session(connector_url, auth_key_id, password)?;
        Ok(Self::from_session(session, signing_key_id))
    }

    /// Create a YubihsmSigner from a yubihsm-rs Session
    pub fn from_session(session: Session, signing_key_id: KeyID) -> Self {
        YubihsmSigner {
            session,
            key_id: signing_key_id,
        }
    }
}

impl Signer for YubihsmSigner {
    fn sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        match self.session.sign_eddsa(self.key_id, msg) {
            Ok(signature) => Ok(Signature::from(&signature[..])),
            Err(e) => Err(e.context(ErrorKind::ProviderError).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    //use ed25519::Signer;
    //use super::YubihsmSigner;
    use super::{generate_key, session, DomainNumber, KeyID};

    /// Default address of connector
    const CONNECTOR_URL: &str = "http://127.0.0.1:12345";

    /// Default authentication key identifier
    const DEFAULT_AUTH_KEYID: KeyID = 1;

    /// Default YubiHSM2 password
    const DEFAULT_PASSWORD: &str = "password";

    /// Key ID to use for test key
    const TEST_KEY_ID: KeyID = 123;

    /// Domain ID for test key
    const TEST_KEY_DOMAIN: DomainNumber = 1;

    /// Label for test key
    const TEST_KEY_LABEL: &str = "Signatory test key";

    #[test]
    fn generates_signature_verifiable_by_dalek() {
        let s = session(CONNECTOR_URL, DEFAULT_AUTH_KEYID, DEFAULT_PASSWORD)
            .expect("can't open session!");

        generate_key(s, TEST_KEY_ID, TEST_KEY_LABEL, TEST_KEY_DOMAIN)
            .expect("can't create test key!");

        // TODO: actually generate and verify signature
    }
}
