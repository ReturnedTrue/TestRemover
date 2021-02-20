// Types
type StaticStr = &'static str;

// Constants
pub const CONFIGURATION_PATH: StaticStr = "trmv.toml";

pub const OUT_DIRECTORY: StaticStr = "out";
pub const SOURCE_DIRECTORY: StaticStr = "source";
pub const TEST_ENDING: StaticStr = ".spec.lua";
pub const IS_VERBOSE: bool = false;

pub const VERSION: StaticStr = "1.0.2";