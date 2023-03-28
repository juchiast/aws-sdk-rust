// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct JoinStorageSessionOutput {}
/// See [`JoinStorageSessionOutput`](crate::output::JoinStorageSessionOutput).
pub mod join_storage_session_output {

    /// A builder for [`JoinStorageSessionOutput`](crate::output::JoinStorageSessionOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`JoinStorageSessionOutput`](crate::output::JoinStorageSessionOutput).
        pub fn build(self) -> crate::output::JoinStorageSessionOutput {
            crate::output::JoinStorageSessionOutput {}
        }
    }
}
impl JoinStorageSessionOutput {
    /// Creates a new builder-style object to manufacture [`JoinStorageSessionOutput`](crate::output::JoinStorageSessionOutput).
    pub fn builder() -> crate::output::join_storage_session_output::Builder {
        crate::output::join_storage_session_output::Builder::default()
    }
}
