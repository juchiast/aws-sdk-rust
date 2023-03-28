// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents an FinSpace environment.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Environment {
    /// <p>The name of the FinSpace environment.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The identifier of the FinSpace environment.</p>
    #[doc(hidden)]
    pub environment_id: std::option::Option<std::string::String>,
    /// <p>The ID of the AWS account in which the FinSpace environment is created.</p>
    #[doc(hidden)]
    pub aws_account_id: std::option::Option<std::string::String>,
    /// <p>The current status of creation of the FinSpace environment.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::EnvironmentStatus>,
    /// <p>The sign-in url for the web application of your FinSpace environment.</p>
    #[doc(hidden)]
    pub environment_url: std::option::Option<std::string::String>,
    /// <p>The description of the FinSpace environment.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of your FinSpace environment.</p>
    #[doc(hidden)]
    pub environment_arn: std::option::Option<std::string::String>,
    /// <p>The url of the integrated FinSpace notebook environment in your web application.</p>
    #[doc(hidden)]
    pub sage_maker_studio_domain_url: std::option::Option<std::string::String>,
    /// <p>The KMS key id used to encrypt in the FinSpace environment.</p>
    #[doc(hidden)]
    pub kms_key_id: std::option::Option<std::string::String>,
    /// <p>The AWS account ID of the dedicated service account associated with your FinSpace environment.</p>
    #[doc(hidden)]
    pub dedicated_service_account_id: std::option::Option<std::string::String>,
    /// <p>The authentication mode for the environment.</p>
    #[doc(hidden)]
    pub federation_mode: std::option::Option<crate::model::FederationMode>,
    /// <p>Configuration information when authentication mode is FEDERATED.</p>
    #[doc(hidden)]
    pub federation_parameters: std::option::Option<crate::model::FederationParameters>,
}
impl Environment {
    /// <p>The name of the FinSpace environment.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The identifier of the FinSpace environment.</p>
    pub fn environment_id(&self) -> std::option::Option<&str> {
        self.environment_id.as_deref()
    }
    /// <p>The ID of the AWS account in which the FinSpace environment is created.</p>
    pub fn aws_account_id(&self) -> std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The current status of creation of the FinSpace environment.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::EnvironmentStatus> {
        self.status.as_ref()
    }
    /// <p>The sign-in url for the web application of your FinSpace environment.</p>
    pub fn environment_url(&self) -> std::option::Option<&str> {
        self.environment_url.as_deref()
    }
    /// <p>The description of the FinSpace environment.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of your FinSpace environment.</p>
    pub fn environment_arn(&self) -> std::option::Option<&str> {
        self.environment_arn.as_deref()
    }
    /// <p>The url of the integrated FinSpace notebook environment in your web application.</p>
    pub fn sage_maker_studio_domain_url(&self) -> std::option::Option<&str> {
        self.sage_maker_studio_domain_url.as_deref()
    }
    /// <p>The KMS key id used to encrypt in the FinSpace environment.</p>
    pub fn kms_key_id(&self) -> std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The AWS account ID of the dedicated service account associated with your FinSpace environment.</p>
    pub fn dedicated_service_account_id(&self) -> std::option::Option<&str> {
        self.dedicated_service_account_id.as_deref()
    }
    /// <p>The authentication mode for the environment.</p>
    pub fn federation_mode(&self) -> std::option::Option<&crate::model::FederationMode> {
        self.federation_mode.as_ref()
    }
    /// <p>Configuration information when authentication mode is FEDERATED.</p>
    pub fn federation_parameters(
        &self,
    ) -> std::option::Option<&crate::model::FederationParameters> {
        self.federation_parameters.as_ref()
    }
}
/// See [`Environment`](crate::model::Environment).
pub mod environment {

    /// A builder for [`Environment`](crate::model::Environment).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) environment_id: std::option::Option<std::string::String>,
        pub(crate) aws_account_id: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::EnvironmentStatus>,
        pub(crate) environment_url: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) environment_arn: std::option::Option<std::string::String>,
        pub(crate) sage_maker_studio_domain_url: std::option::Option<std::string::String>,
        pub(crate) kms_key_id: std::option::Option<std::string::String>,
        pub(crate) dedicated_service_account_id: std::option::Option<std::string::String>,
        pub(crate) federation_mode: std::option::Option<crate::model::FederationMode>,
        pub(crate) federation_parameters: std::option::Option<crate::model::FederationParameters>,
    }
    impl Builder {
        /// <p>The name of the FinSpace environment.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the FinSpace environment.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_id = Some(input.into());
            self
        }
        /// <p>The identifier of the FinSpace environment.</p>
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_id = input;
            self
        }
        /// <p>The ID of the AWS account in which the FinSpace environment is created.</p>
        pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.aws_account_id = Some(input.into());
            self
        }
        /// <p>The ID of the AWS account in which the FinSpace environment is created.</p>
        pub fn set_aws_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.aws_account_id = input;
            self
        }
        /// <p>The current status of creation of the FinSpace environment.</p>
        pub fn status(mut self, input: crate::model::EnvironmentStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The current status of creation of the FinSpace environment.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::EnvironmentStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>The sign-in url for the web application of your FinSpace environment.</p>
        pub fn environment_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_url = Some(input.into());
            self
        }
        /// <p>The sign-in url for the web application of your FinSpace environment.</p>
        pub fn set_environment_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_url = input;
            self
        }
        /// <p>The description of the FinSpace environment.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The description of the FinSpace environment.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of your FinSpace environment.</p>
        pub fn environment_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of your FinSpace environment.</p>
        pub fn set_environment_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_arn = input;
            self
        }
        /// <p>The url of the integrated FinSpace notebook environment in your web application.</p>
        pub fn sage_maker_studio_domain_url(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.sage_maker_studio_domain_url = Some(input.into());
            self
        }
        /// <p>The url of the integrated FinSpace notebook environment in your web application.</p>
        pub fn set_sage_maker_studio_domain_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.sage_maker_studio_domain_url = input;
            self
        }
        /// <p>The KMS key id used to encrypt in the FinSpace environment.</p>
        pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.kms_key_id = Some(input.into());
            self
        }
        /// <p>The KMS key id used to encrypt in the FinSpace environment.</p>
        pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.kms_key_id = input;
            self
        }
        /// <p>The AWS account ID of the dedicated service account associated with your FinSpace environment.</p>
        pub fn dedicated_service_account_id(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.dedicated_service_account_id = Some(input.into());
            self
        }
        /// <p>The AWS account ID of the dedicated service account associated with your FinSpace environment.</p>
        pub fn set_dedicated_service_account_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.dedicated_service_account_id = input;
            self
        }
        /// <p>The authentication mode for the environment.</p>
        pub fn federation_mode(mut self, input: crate::model::FederationMode) -> Self {
            self.federation_mode = Some(input);
            self
        }
        /// <p>The authentication mode for the environment.</p>
        pub fn set_federation_mode(
            mut self,
            input: std::option::Option<crate::model::FederationMode>,
        ) -> Self {
            self.federation_mode = input;
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn federation_parameters(mut self, input: crate::model::FederationParameters) -> Self {
            self.federation_parameters = Some(input);
            self
        }
        /// <p>Configuration information when authentication mode is FEDERATED.</p>
        pub fn set_federation_parameters(
            mut self,
            input: std::option::Option<crate::model::FederationParameters>,
        ) -> Self {
            self.federation_parameters = input;
            self
        }
        /// Consumes the builder and constructs a [`Environment`](crate::model::Environment).
        pub fn build(self) -> crate::model::Environment {
            crate::model::Environment {
                name: self.name,
                environment_id: self.environment_id,
                aws_account_id: self.aws_account_id,
                status: self.status,
                environment_url: self.environment_url,
                description: self.description,
                environment_arn: self.environment_arn,
                sage_maker_studio_domain_url: self.sage_maker_studio_domain_url,
                kms_key_id: self.kms_key_id,
                dedicated_service_account_id: self.dedicated_service_account_id,
                federation_mode: self.federation_mode,
                federation_parameters: self.federation_parameters,
            }
        }
    }
}
impl Environment {
    /// Creates a new builder-style object to manufacture [`Environment`](crate::model::Environment).
    pub fn builder() -> crate::model::environment::Builder {
        crate::model::environment::Builder::default()
    }
}

/// <p>Configuration information when authentication mode is FEDERATED.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct FederationParameters {
    /// <p>SAML 2.0 Metadata document from identity provider (IdP).</p>
    #[doc(hidden)]
    pub saml_metadata_document: std::option::Option<std::string::String>,
    /// <p>Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).</p>
    #[doc(hidden)]
    pub saml_metadata_url: std::option::Option<std::string::String>,
    /// <p>The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration (IdP).</p>
    #[doc(hidden)]
    pub application_call_back_url: std::option::Option<std::string::String>,
    /// <p>The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.</p>
    #[doc(hidden)]
    pub federation_urn: std::option::Option<std::string::String>,
    /// <p>Name of the identity provider (IdP).</p>
    #[doc(hidden)]
    pub federation_provider_name: std::option::Option<std::string::String>,
    /// <p>SAML attribute name and value. The name must always be <code>Email</code> and the value should be set to the attribute definition in which user email is set. For example, name would be <code>Email</code> and value <code>http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress</code>. Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.</p>
    #[doc(hidden)]
    pub attribute_map:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl FederationParameters {
    /// <p>SAML 2.0 Metadata document from identity provider (IdP).</p>
    pub fn saml_metadata_document(&self) -> std::option::Option<&str> {
        self.saml_metadata_document.as_deref()
    }
    /// <p>Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).</p>
    pub fn saml_metadata_url(&self) -> std::option::Option<&str> {
        self.saml_metadata_url.as_deref()
    }
    /// <p>The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration (IdP).</p>
    pub fn application_call_back_url(&self) -> std::option::Option<&str> {
        self.application_call_back_url.as_deref()
    }
    /// <p>The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.</p>
    pub fn federation_urn(&self) -> std::option::Option<&str> {
        self.federation_urn.as_deref()
    }
    /// <p>Name of the identity provider (IdP).</p>
    pub fn federation_provider_name(&self) -> std::option::Option<&str> {
        self.federation_provider_name.as_deref()
    }
    /// <p>SAML attribute name and value. The name must always be <code>Email</code> and the value should be set to the attribute definition in which user email is set. For example, name would be <code>Email</code> and value <code>http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress</code>. Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.</p>
    pub fn attribute_map(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.attribute_map.as_ref()
    }
}
/// See [`FederationParameters`](crate::model::FederationParameters).
pub mod federation_parameters {

    /// A builder for [`FederationParameters`](crate::model::FederationParameters).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) saml_metadata_document: std::option::Option<std::string::String>,
        pub(crate) saml_metadata_url: std::option::Option<std::string::String>,
        pub(crate) application_call_back_url: std::option::Option<std::string::String>,
        pub(crate) federation_urn: std::option::Option<std::string::String>,
        pub(crate) federation_provider_name: std::option::Option<std::string::String>,
        pub(crate) attribute_map: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// <p>SAML 2.0 Metadata document from identity provider (IdP).</p>
        pub fn saml_metadata_document(mut self, input: impl Into<std::string::String>) -> Self {
            self.saml_metadata_document = Some(input.into());
            self
        }
        /// <p>SAML 2.0 Metadata document from identity provider (IdP).</p>
        pub fn set_saml_metadata_document(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.saml_metadata_document = input;
            self
        }
        /// <p>Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).</p>
        pub fn saml_metadata_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.saml_metadata_url = Some(input.into());
            self
        }
        /// <p>Provide the metadata URL from your SAML 2.0 compliant identity provider (IdP).</p>
        pub fn set_saml_metadata_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.saml_metadata_url = input;
            self
        }
        /// <p>The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration (IdP).</p>
        pub fn application_call_back_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_call_back_url = Some(input.into());
            self
        }
        /// <p>The redirect or sign-in URL that should be entered into the SAML 2.0 compliant identity provider configuration (IdP).</p>
        pub fn set_application_call_back_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_call_back_url = input;
            self
        }
        /// <p>The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.</p>
        pub fn federation_urn(mut self, input: impl Into<std::string::String>) -> Self {
            self.federation_urn = Some(input.into());
            self
        }
        /// <p>The Uniform Resource Name (URN). Also referred as Service Provider URN or Audience URI or Service Provider Entity ID.</p>
        pub fn set_federation_urn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.federation_urn = input;
            self
        }
        /// <p>Name of the identity provider (IdP).</p>
        pub fn federation_provider_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.federation_provider_name = Some(input.into());
            self
        }
        /// <p>Name of the identity provider (IdP).</p>
        pub fn set_federation_provider_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.federation_provider_name = input;
            self
        }
        /// Adds a key-value pair to `attribute_map`.
        ///
        /// To override the contents of this collection use [`set_attribute_map`](Self::set_attribute_map).
        ///
        /// <p>SAML attribute name and value. The name must always be <code>Email</code> and the value should be set to the attribute definition in which user email is set. For example, name would be <code>Email</code> and value <code>http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress</code>. Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.</p>
        pub fn attribute_map(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.attribute_map.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.attribute_map = Some(hash_map);
            self
        }
        /// <p>SAML attribute name and value. The name must always be <code>Email</code> and the value should be set to the attribute definition in which user email is set. For example, name would be <code>Email</code> and value <code>http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress</code>. Please check your SAML 2.0 compliant identity provider (IdP) documentation for details.</p>
        pub fn set_attribute_map(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.attribute_map = input;
            self
        }
        /// Consumes the builder and constructs a [`FederationParameters`](crate::model::FederationParameters).
        pub fn build(self) -> crate::model::FederationParameters {
            crate::model::FederationParameters {
                saml_metadata_document: self.saml_metadata_document,
                saml_metadata_url: self.saml_metadata_url,
                application_call_back_url: self.application_call_back_url,
                federation_urn: self.federation_urn,
                federation_provider_name: self.federation_provider_name,
                attribute_map: self.attribute_map,
            }
        }
    }
}
impl FederationParameters {
    /// Creates a new builder-style object to manufacture [`FederationParameters`](crate::model::FederationParameters).
    pub fn builder() -> crate::model::federation_parameters::Builder {
        crate::model::federation_parameters::Builder::default()
    }
}

/// When writing a match expression against `FederationMode`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let federationmode = unimplemented!();
/// match federationmode {
///     FederationMode::Federated => { /* ... */ },
///     FederationMode::Local => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `federationmode` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `FederationMode::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `FederationMode::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `FederationMode::NewFeature` is defined.
/// Specifically, when `federationmode` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `FederationMode::NewFeature` also yielding `"NewFeature"`.
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
pub enum FederationMode {
    #[allow(missing_docs)] // documentation missing in model
    Federated,
    #[allow(missing_docs)] // documentation missing in model
    Local,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for FederationMode {
    fn from(s: &str) -> Self {
        match s {
            "FEDERATED" => FederationMode::Federated,
            "LOCAL" => FederationMode::Local,
            other => FederationMode::Unknown(crate::types::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl std::str::FromStr for FederationMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(FederationMode::from(s))
    }
}
impl FederationMode {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            FederationMode::Federated => "FEDERATED",
            FederationMode::Local => "LOCAL",
            FederationMode::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &["FEDERATED", "LOCAL"]
    }
}
impl AsRef<str> for FederationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// When writing a match expression against `EnvironmentStatus`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let environmentstatus = unimplemented!();
/// match environmentstatus {
///     EnvironmentStatus::Created => { /* ... */ },
///     EnvironmentStatus::CreateRequested => { /* ... */ },
///     EnvironmentStatus::Creating => { /* ... */ },
///     EnvironmentStatus::Deleted => { /* ... */ },
///     EnvironmentStatus::DeleteRequested => { /* ... */ },
///     EnvironmentStatus::Deleting => { /* ... */ },
///     EnvironmentStatus::FailedCreation => { /* ... */ },
///     EnvironmentStatus::FailedDeletion => { /* ... */ },
///     EnvironmentStatus::RetryDeletion => { /* ... */ },
///     EnvironmentStatus::Suspended => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `environmentstatus` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `EnvironmentStatus::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `EnvironmentStatus::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `EnvironmentStatus::NewFeature` is defined.
/// Specifically, when `environmentstatus` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `EnvironmentStatus::NewFeature` also yielding `"NewFeature"`.
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
pub enum EnvironmentStatus {
    #[allow(missing_docs)] // documentation missing in model
    Created,
    #[allow(missing_docs)] // documentation missing in model
    CreateRequested,
    #[allow(missing_docs)] // documentation missing in model
    Creating,
    #[allow(missing_docs)] // documentation missing in model
    Deleted,
    #[allow(missing_docs)] // documentation missing in model
    DeleteRequested,
    #[allow(missing_docs)] // documentation missing in model
    Deleting,
    #[allow(missing_docs)] // documentation missing in model
    FailedCreation,
    #[allow(missing_docs)] // documentation missing in model
    FailedDeletion,
    #[allow(missing_docs)] // documentation missing in model
    RetryDeletion,
    #[allow(missing_docs)] // documentation missing in model
    Suspended,
    /// `Unknown` contains new variants that have been added since this code was generated.
    Unknown(crate::types::UnknownVariantValue),
}
impl std::convert::From<&str> for EnvironmentStatus {
    fn from(s: &str) -> Self {
        match s {
            "CREATED" => EnvironmentStatus::Created,
            "CREATE_REQUESTED" => EnvironmentStatus::CreateRequested,
            "CREATING" => EnvironmentStatus::Creating,
            "DELETED" => EnvironmentStatus::Deleted,
            "DELETE_REQUESTED" => EnvironmentStatus::DeleteRequested,
            "DELETING" => EnvironmentStatus::Deleting,
            "FAILED_CREATION" => EnvironmentStatus::FailedCreation,
            "FAILED_DELETION" => EnvironmentStatus::FailedDeletion,
            "RETRY_DELETION" => EnvironmentStatus::RetryDeletion,
            "SUSPENDED" => EnvironmentStatus::Suspended,
            other => {
                EnvironmentStatus::Unknown(crate::types::UnknownVariantValue(other.to_owned()))
            }
        }
    }
}
impl std::str::FromStr for EnvironmentStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EnvironmentStatus::from(s))
    }
}
impl EnvironmentStatus {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            EnvironmentStatus::Created => "CREATED",
            EnvironmentStatus::CreateRequested => "CREATE_REQUESTED",
            EnvironmentStatus::Creating => "CREATING",
            EnvironmentStatus::Deleted => "DELETED",
            EnvironmentStatus::DeleteRequested => "DELETE_REQUESTED",
            EnvironmentStatus::Deleting => "DELETING",
            EnvironmentStatus::FailedCreation => "FAILED_CREATION",
            EnvironmentStatus::FailedDeletion => "FAILED_DELETION",
            EnvironmentStatus::RetryDeletion => "RETRY_DELETION",
            EnvironmentStatus::Suspended => "SUSPENDED",
            EnvironmentStatus::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "CREATED",
            "CREATE_REQUESTED",
            "CREATING",
            "DELETED",
            "DELETE_REQUESTED",
            "DELETING",
            "FAILED_CREATION",
            "FAILED_DELETION",
            "RETRY_DELETION",
            "SUSPENDED",
        ]
    }
}
impl AsRef<str> for EnvironmentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Configuration information for the superuser.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SuperuserParameters {
    /// <p>The email address of the superuser.</p>
    #[doc(hidden)]
    pub email_address: std::option::Option<std::string::String>,
    /// <p>The first name of the superuser.</p>
    #[doc(hidden)]
    pub first_name: std::option::Option<std::string::String>,
    /// <p>The last name of the superuser.</p>
    #[doc(hidden)]
    pub last_name: std::option::Option<std::string::String>,
}
impl SuperuserParameters {
    /// <p>The email address of the superuser.</p>
    pub fn email_address(&self) -> std::option::Option<&str> {
        self.email_address.as_deref()
    }
    /// <p>The first name of the superuser.</p>
    pub fn first_name(&self) -> std::option::Option<&str> {
        self.first_name.as_deref()
    }
    /// <p>The last name of the superuser.</p>
    pub fn last_name(&self) -> std::option::Option<&str> {
        self.last_name.as_deref()
    }
}
impl std::fmt::Debug for SuperuserParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SuperuserParameters");
        formatter.field("email_address", &"*** Sensitive Data Redacted ***");
        formatter.field("first_name", &self.first_name);
        formatter.field("last_name", &self.last_name);
        formatter.finish()
    }
}
/// See [`SuperuserParameters`](crate::model::SuperuserParameters).
pub mod superuser_parameters {

    /// A builder for [`SuperuserParameters`](crate::model::SuperuserParameters).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) email_address: std::option::Option<std::string::String>,
        pub(crate) first_name: std::option::Option<std::string::String>,
        pub(crate) last_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The email address of the superuser.</p>
        pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.email_address = Some(input.into());
            self
        }
        /// <p>The email address of the superuser.</p>
        pub fn set_email_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.email_address = input;
            self
        }
        /// <p>The first name of the superuser.</p>
        pub fn first_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.first_name = Some(input.into());
            self
        }
        /// <p>The first name of the superuser.</p>
        pub fn set_first_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.first_name = input;
            self
        }
        /// <p>The last name of the superuser.</p>
        pub fn last_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.last_name = Some(input.into());
            self
        }
        /// <p>The last name of the superuser.</p>
        pub fn set_last_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.last_name = input;
            self
        }
        /// Consumes the builder and constructs a [`SuperuserParameters`](crate::model::SuperuserParameters).
        pub fn build(self) -> crate::model::SuperuserParameters {
            crate::model::SuperuserParameters {
                email_address: self.email_address,
                first_name: self.first_name,
                last_name: self.last_name,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("email_address", &"*** Sensitive Data Redacted ***");
            formatter.field("first_name", &self.first_name);
            formatter.field("last_name", &self.last_name);
            formatter.finish()
        }
    }
}
impl SuperuserParameters {
    /// Creates a new builder-style object to manufacture [`SuperuserParameters`](crate::model::SuperuserParameters).
    pub fn builder() -> crate::model::superuser_parameters::Builder {
        crate::model::superuser_parameters::Builder::default()
    }
}
