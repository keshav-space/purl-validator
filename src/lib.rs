use fst::Set;
use once_cell::sync::Lazy;

static FST_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/purls.fst"));

static VALIDATOR: Lazy<Set<&[u8]>> =
    Lazy::new(|| Set::new(FST_BYTES).expect("Failed to load FST from embedded bytes"));

pub fn validate(packageurl: &str) -> bool {
    let trimmed_packageurl = packageurl.trim_end_matches("/");
    VALIDATOR.contains(trimmed_packageurl)
}
