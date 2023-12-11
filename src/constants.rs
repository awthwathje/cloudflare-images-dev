// default values for env vars
pub const DEFAULT_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "3030";
pub const DEFAULT_ACCOUNT_ID: &str = "account_id";
pub const DEFAULT_ACCOUNT_HASH: &str = "account_hash";
pub const DEFAULT_VARIANTS: &str = "variant1,variant2,variant3";

// non-env related
pub const FILE_STORAGE_DIR: &str = "./.files/";
pub const MAX_FILE_SIZE: u64 = 50 * 1_048_576; // 50 MB
