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
//! <p>Amazon GameLift provides solutions for hosting session-based multiplayer game servers in the
//! cloud, including tools for deploying, operating, and scaling game servers. Built on
//! Amazon Web Services global computing infrastructure, GameLift helps you deliver high-performance,
//! high-reliability, low-cost game servers while dynamically scaling your resource usage to
//! meet player demand. </p>
//! <p>
//! <b>About GameLift solutions</b>
//! </p>
//! <p>Get more information on these GameLift solutions in the <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/">GameLift Developer Guide</a>.</p>
//! <ul>
//! <li>
//! <p>GameLift managed hosting -- GameLift offers a fully managed service to set up
//! and maintain computing machines for hosting, manage game session and player
//! session life cycle, and handle security, storage, and performance tracking. You
//! can use automatic scaling tools to balance player demand and hosting costs,
//! configure your game session management to minimize player latency, and add
//! FlexMatch for matchmaking.</p>
//! </li>
//! <li>
//! <p>Managed hosting with Realtime Servers -- With GameLift Realtime Servers, you can quickly configure
//! and set up ready-to-go game servers for your game. Realtime Servers provides a game server
//! framework with core GameLift infrastructure already built in. Then use the full
//! range of GameLift managed hosting features, including FlexMatch, for your
//! game.</p>
//! </li>
//! <li>
//! <p>GameLift FleetIQ -- Use GameLift FleetIQ as a standalone service while hosting your games using EC2
//! instances and Auto Scaling groups. GameLift FleetIQ provides optimizations for game
//! hosting, including boosting the viability of low-cost Spot Instances gaming. For
//! a complete solution, pair the GameLift FleetIQ and FlexMatch standalone services.</p>
//! </li>
//! <li>
//! <p>GameLift FlexMatch -- Add matchmaking to your game hosting solution. FlexMatch is a
//! customizable matchmaking service for multiplayer games. Use FlexMatch as
//! integrated with GameLift managed hosting or incorporate FlexMatch as a standalone
//! service into your own hosting solution.</p>
//! </li>
//! </ul>
//! <p>
//! <b>About this API Reference</b>
//! </p>
//! <p>This reference guide describes the low-level service API for Amazon GameLift. With each topic
//! in this guide, you can find links to language-specific SDK guides and the Amazon Web Services CLI
//! reference. Useful links:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-awssdk.html">GameLift API
//! operations listed by tasks</a>
//! </p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-components.html"> GameLift tools
//! and resources</a>
//! </p>
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
    aws_http::user_agent::ApiMetadata::new("gamelift", PKG_VERSION);
pub use aws_credential_types::Credentials;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
#[doc(inline)]
pub use client::Client;
