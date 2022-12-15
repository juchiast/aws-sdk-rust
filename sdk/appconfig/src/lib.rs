#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Use AppConfig, a capability of Amazon Web Services Systems Manager, to create, manage, and quickly
//! deploy application configurations. AppConfig supports controlled deployments to
//! applications of any size and includes built-in validation checks and monitoring. You can
//! use AppConfig with applications hosted on Amazon EC2 instances, Lambda, containers,
//! mobile applications, or IoT devices.</p>
//! <p>To prevent errors when deploying application configurations, especially for production
//! systems where a simple typo could cause an unexpected outage, AppConfig includes
//! validators. A validator provides a syntactic or semantic check to ensure that the
//! configuration you want to deploy works as intended. To validate your application
//! configuration data, you provide a schema or an Amazon Web Services Lambda function that runs against
//! the configuration. The configuration deployment or update can only proceed when the
//! configuration data is valid.</p>
//! <p>During a configuration deployment, AppConfig monitors the application to
//! ensure that the deployment is successful. If the system encounters an error, AppConfig rolls back the change to minimize impact for your application users. You can
//! configure a deployment strategy for each application or environment that includes
//! deployment criteria, including velocity, bake time, and alarms to monitor. Similar to error
//! monitoring, if a deployment triggers an alarm, AppConfig automatically rolls back
//! to the previous version. </p>
//! <p>AppConfig supports multiple use cases. Here are some examples:</p>
//! <ul>
//! <li>
//! <p>
//! <b>Feature flags</b>: Use AppConfig to turn on
//! new features that require a timely deployment, such as a product launch or
//! announcement. </p>
//! </li>
//! <li>
//! <p>
//! <b>Application tuning</b>: Use AppConfig to
//! carefully introduce changes to your application that can only be tested with
//! production traffic.</p>
//! </li>
//! <li>
//! <p>
//! <b>Allow list</b>: Use AppConfig to allow
//! premium subscribers to access paid content. </p>
//! </li>
//! <li>
//! <p>
//! <b>Operational issues</b>: Use AppConfig to
//! reduce stress on your application when a dependency or other external factor impacts
//! the system.</p>
//! </li>
//! </ul>
//! <p>This reference is intended to be used with the <a href="http://docs.aws.amazon.com/appconfig/latest/userguide/what-is-appconfig.html">AppConfig User
//! Guide</a>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod aws_endpoint;

mod http_serde;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("appconfig", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
