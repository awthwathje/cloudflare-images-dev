// default values for env vars
pub const DEFAULT_HOSTNAME: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "3030";
pub const DEFAULT_ACCOUNT_ID: &str = "account-id";
pub const DEFAULT_ACCOUNT_HASH: &str = "account-hash";
pub const DEFAULT_VARIANTS: &str = "variant1_1920_1080,variant2_640_480,variant3_40_40";

// non-env related
pub const FILE_STORAGE_DIR: &str = "./.files/";
pub const MAX_FILE_SIZE: u64 = 50 * 1_048_576; // 50 MB
