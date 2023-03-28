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
//! <p>Welcome to the Routing Control (Recovery Cluster) API Reference Guide for Amazon Route 53 Application Recovery Controller.</p>
//! <p>With Route 53 ARC, you can use routing control with extreme reliability to
//! recover applications by rerouting traffic across
//! Availability Zones or Amazon Web Services Regions. Routing controls are simple on/off switches hosted
//! on a highly available cluster in Route 53 ARC. A cluster provides a set of five redundant Regional endpoints against which you
//! can run API calls to get or update the state of routing controls. To implement failover, you set
//! one routing control On and another one Off, to reroute traffic from one Availability Zone or Amazon Web Services Region
//! to another. </p>
//! <p>
//! <i>Be aware that you must specify a Regional endpoint for a cluster when you work with API cluster operations
//! to get or update routing control states in Route 53 ARC.</i> In addition, you must specify the US West (Oregon) Region
//! for Route 53 ARC API calls. For example, use the parameter <code>--region us-west-2</code> with AWS CLI commands.
//! For more information, see
//! <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/routing-control.update.api.html">
//! Get and update routing control states using the API</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p>
//! <p>This API guide includes information about the API operations for how to get and update routing control states
//! in Route 53 ARC. To work with routing control in Route 53 ARC, you must first create the required components (clusters, control
//! panels, and routing controls) using the recovery cluster configuration API.</p>
//! <p>For more information about working with routing control in Route 53 ARC, see the following:</p>
//! <ul>
//! <li>
//! <p>Create clusters, control panels, and routing controls by using API operations. For more information,
//! see the <a href="https://docs.aws.amazon.com/recovery-cluster/latest/api/">Recovery Control Configuration API Reference Guide for Amazon Route 53 Application Recovery Controller</a>.</p>
//! </li>
//! <li>
//! <p>Learn about the components in recovery control, including clusters,
//! routing controls, and control panels, and how to work with Route 53 ARC in the Amazon Web Services console. For more
//! information, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/introduction-components.html#introduction-components-routing">
//! Recovery control components</a> in the Amazon Route 53 Application Recovery Controller Developer Guide.</p>
//! </li>
//! <li>
//! <p>Route 53 ARC also provides readiness checks that continually audit resources to help make sure that your
//! applications are scaled and ready to handle failover traffic. For more information about
//! the related API operations, see the <a href="https://docs.aws.amazon.com/recovery-readiness/latest/api/">Recovery Readiness API Reference Guide for Amazon Route 53 Application Recovery Controller</a>.</p>
//! </li>
//! <li>
//! <p>For more information about creating resilient applications and preparing for
//! recovery readiness with Route 53 ARC, see the <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/">Amazon Route 53 Application Recovery Controller Developer Guide</a>.</p>
//! </li>
//! </ul>
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
    aws_http::user_agent::ApiMetadata::new("route53recoverycluster", PKG_VERSION);
pub use aws_credential_types::Credentials;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
#[doc(inline)]
pub use client::Client;
