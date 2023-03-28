// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A summary of information about a AWS IoT Device Management web application.</p> <note>
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p>
/// </note>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ApplicationSummary {
    /// <p>The unique Id of the web application.</p>
    #[doc(hidden)]
    pub application_id: std::option::Option<std::string::String>,
    /// <p>The name of the web application.</p>
    #[doc(hidden)]
    pub application_name: std::option::Option<std::string::String>,
    /// <p>An optional description of the web application.</p>
    #[doc(hidden)]
    pub application_description: std::option::Option<std::string::String>,
    /// <p>The URL of the web application.</p>
    #[doc(hidden)]
    pub application_url: std::option::Option<std::string::String>,
    /// <p>The date (in Unix epoch time) when the web application was created.</p>
    #[doc(hidden)]
    pub application_creation_date: i64,
    /// <p>The date (in Unix epoch time) when the web application was last updated.</p>
    #[doc(hidden)]
    pub application_last_update_date: i64,
    /// <p>The current state of the web application.</p>
    #[doc(hidden)]
    pub application_state: std::option::Option<crate::model::ApplicationState>,
}
impl ApplicationSummary {
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(&self) -> std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The name of the web application.</p>
    pub fn application_name(&self) -> std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>An optional description of the web application.</p>
    pub fn application_description(&self) -> std::option::Option<&str> {
        self.application_description.as_deref()
    }
    /// <p>The URL of the web application.</p>
    pub fn application_url(&self) -> std::option::Option<&str> {
        self.application_url.as_deref()
    }
    /// <p>The date (in Unix epoch time) when the web application was created.</p>
    pub fn application_creation_date(&self) -> i64 {
        self.application_creation_date
    }
    /// <p>The date (in Unix epoch time) when the web application was last updated.</p>
    pub fn application_last_update_date(&self) -> i64 {
        self.application_last_update_date
    }
    /// <p>The current state of the web application.</p>
    pub fn application_state(&self) -> std::option::Option<&crate::model::ApplicationState> {
        self.application_state.as_ref()
    }
}
/// See [`ApplicationSummary`](crate::model::ApplicationSummary).
pub mod application_summary {

    /// A builder for [`ApplicationSummary`](crate::model::ApplicationSummary).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_id: std::option::Option<std::string::String>,
        pub(crate) application_name: std::option::Option<std::string::String>,
        pub(crate) application_description: std::option::Option<std::string::String>,
        pub(crate) application_url: std::option::Option<std::string::String>,
        pub(crate) application_creation_date: std::option::Option<i64>,
        pub(crate) application_last_update_date: std::option::Option<i64>,
        pub(crate) application_state: std::option::Option<crate::model::ApplicationState>,
    }
    impl Builder {
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_id = Some(input.into());
            self
        }
        /// <p>The unique Id of the web application.</p>
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_id = input;
            self
        }
        /// <p>The name of the web application.</p>
        pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_name = Some(input.into());
            self
        }
        /// <p>The name of the web application.</p>
        pub fn set_application_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_name = input;
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_description = Some(input.into());
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn set_application_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_description = input;
            self
        }
        /// <p>The URL of the web application.</p>
        pub fn application_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_url = Some(input.into());
            self
        }
        /// <p>The URL of the web application.</p>
        pub fn set_application_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_url = input;
            self
        }
        /// <p>The date (in Unix epoch time) when the web application was created.</p>
        pub fn application_creation_date(mut self, input: i64) -> Self {
            self.application_creation_date = Some(input);
            self
        }
        /// <p>The date (in Unix epoch time) when the web application was created.</p>
        pub fn set_application_creation_date(mut self, input: std::option::Option<i64>) -> Self {
            self.application_creation_date = input;
            self
        }
        /// <p>The date (in Unix epoch time) when the web application was last updated.</p>
        pub fn application_last_update_date(mut self, input: i64) -> Self {
            self.application_last_update_date = Some(input);
            self
        }
        /// <p>The date (in Unix epoch time) when the web application was last updated.</p>
        pub fn set_application_last_update_date(mut self, input: std::option::Option<i64>) -> Self {
            self.application_last_update_date = input;
            self
        }
        /// <p>The current state of the web application.</p>
        pub fn application_state(mut self, input: crate::model::ApplicationState) -> Self {
            self.application_state = Some(input);
            self
        }
        /// <p>The current state of the web application.</p>
        pub fn set_application_state(
            mut self,
            input: std::option::Option<crate::model::ApplicationState>,
        ) -> Self {
            self.application_state = input;
            self
        }
        /// Consumes the builder and constructs a [`ApplicationSummary`](crate::model::ApplicationSummary).
        pub fn build(self) -> crate::model::ApplicationSummary {
            crate::model::ApplicationSummary {
                application_id: self.application_id,
                application_name: self.application_name,
                application_description: self.application_description,
                application_url: self.application_url,
                application_creation_date: self.application_creation_date.unwrap_or_default(),
                application_last_update_date: self.application_last_update_date.unwrap_or_default(),
                application_state: self.application_state,
            }
        }
    }
}
impl ApplicationSummary {
    /// Creates a new builder-style object to manufacture [`ApplicationSummary`](crate::model::ApplicationSummary).
    pub fn builder() -> crate::model::application_summary::Builder {
        crate::model::application_summary::Builder::default()
    }
}

/// When writing a match expression against `ApplicationState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let applicationstate = unimplemented!();
/// match applicationstate {
///     ApplicationState::Active => { /* ... */ },
///     ApplicationState::CreateFailed => { /* ... */ },
///     ApplicationState::Creating => { /* ... */ },
///     ApplicationState::DeleteFailed => { /* ... */ },
///     ApplicationState::Deleting => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `applicationstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `ApplicationState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `ApplicationState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `ApplicationState::NewFeature` is defined.
/// Specifically, when `applicationstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `ApplicationState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ApplicationState {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    CreateFailed,
    #[allow(missing_docs)] // documentation missing in model
    Creating,
    #[allow(missing_docs)] // documentation missing in model
    DeleteFailed,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for ApplicationState {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => ApplicationState::Active,
            "CREATE_FAILED" => ApplicationState::CreateFailed,
            "CREATING" => ApplicationState::Creating,
            "DELETE_FAILED" => ApplicationState::DeleteFailed,
            "DELETING" => ApplicationState::Deleting,
            other => ApplicationState::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for ApplicationState {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ApplicationState::from(s))
    }
}
impl ApplicationState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ApplicationState::Active => "ACTIVE",
            ApplicationState::CreateFailed => "CREATE_FAILED",
            ApplicationState::Creating => "CREATING",
            ApplicationState::DeleteFailed => "DELETE_FAILED",
            ApplicationState::Deleting => "DELETING",
            ApplicationState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "ACTIVE",
            "CREATE_FAILED",
            "CREATING",
            "DELETE_FAILED",
            "DELETING",
        ]
    }
}
impl AsRef<str> for ApplicationState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
