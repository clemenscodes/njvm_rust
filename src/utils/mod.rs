pub mod check_ninja_format;
pub use check_ninja_format::check_ninja_format;
pub mod check_variables;
pub use check_variables::check_variables;
pub mod check_instructions;
pub use check_instructions::check_instructions;
pub mod check_ninja_version;
pub use check_ninja_version::{check_ninja_version, VERSION};
pub mod set_ninja_version;
pub use set_ninja_version::set_ninja_version;
pub mod read_file;
pub use read_file::read_file;
pub mod split_file_metadata;
pub use split_file_metadata::split_file_metadata;
pub mod verify_arg;
pub use verify_arg::verify_arg;
pub mod fatal_error;
pub use fatal_error::fatal_error;
pub mod unknown_arg;
pub use unknown_arg::unknown_arg;
