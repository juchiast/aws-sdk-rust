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
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Pinpoint Email Service</fullname>
//! <p>Welcome to the <i>Amazon Pinpoint Email API Reference</i>. This guide provides
//! information about the Amazon Pinpoint Email API (version 1.0), including supported
//! operations, data types, parameters, and schemas.</p>
//! <p>
//! <a href="https://aws.amazon.com/pinpoint">Amazon Pinpoint</a> is an AWS service that you
//! can use to engage with your customers across multiple messaging channels. You can use
//! Amazon Pinpoint to send email, SMS text messages, voice messages, and push notifications. The
//! Amazon Pinpoint Email API provides programmatic access to options that are unique to the
//! email channel and supplement the options provided by the Amazon Pinpoint API.</p>
//! <p>If you're new to Amazon Pinpoint, you might find it helpful to also review the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">Amazon
//! Pinpoint Developer Guide</a>. The <i>Amazon Pinpoint Developer
//! Guide</i> provides tutorials, code samples, and procedures that demonstrate
//! how to use Amazon Pinpoint features programmatically and how to integrate Amazon Pinpoint functionality into
//! mobile apps and other types of applications. The guide also provides information about
//! key topics such as Amazon Pinpoint integration with other AWS services and the limits that apply
//! to using the service.</p>
//! <p>The Amazon Pinpoint Email API is available in several AWS Regions and it provides an endpoint
//! for each of these Regions. For a list of all the Regions and endpoints where the API is
//! currently available, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#pinpoint_region">AWS Service Endpoints</a> in
//! the <i>Amazon Web Services General Reference</i>. To learn more about AWS Regions, see
//! <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html">Managing AWS
//! Regions</a> in the <i>Amazon Web Services General Reference</i>.</p>
//! <p>In each Region, AWS maintains multiple Availability Zones. These Availability Zones
//! are physically isolated from each other, but are united by private, low-latency,
//! high-throughput, and highly redundant network connections. These Availability Zones
//! enable us to provide very high levels of availability and redundancy, while also
//! minimizing latency. To learn more about the number of Availability Zones that are
//! available in each Region, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/">AWS Global Infrastructure</a>.</p>
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

/// Endpoint resolution functionality
pub mod endpoint;

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

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::endpoint::Endpoint;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("pinpointemail", PKG_VERSION);
pub use aws_credential_types::Credentials;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
#[doc(inline)]
pub use client::Client;
