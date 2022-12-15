// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon Sagemaker Edge Manager
///
/// Client for invoking operations on Amazon Sagemaker Edge Manager. Each operation on Amazon Sagemaker Edge Manager is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_sagemakeredge::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::retry::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_sagemakeredge::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_sagemakeredge::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`GetDeployments`](crate::client::fluent_builders::GetDeployments) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`device_name(impl Into<String>)`](crate::client::fluent_builders::GetDeployments::device_name) / [`set_device_name(Option<String>)`](crate::client::fluent_builders::GetDeployments::set_device_name): <p>The unique name of the device you want to get the configuration of active deployments from.</p>
    ///   - [`device_fleet_name(impl Into<String>)`](crate::client::fluent_builders::GetDeployments::device_fleet_name) / [`set_device_fleet_name(Option<String>)`](crate::client::fluent_builders::GetDeployments::set_device_fleet_name): <p>The name of the fleet that the device belongs to.</p>
    /// - On success, responds with [`GetDeploymentsOutput`](crate::output::GetDeploymentsOutput) with field(s):
    ///   - [`deployments(Option<Vec<EdgeDeployment>>)`](crate::output::GetDeploymentsOutput::deployments): <p>Returns a list of the configurations of the active deployments on the device.</p>
    /// - On failure, responds with [`SdkError<GetDeploymentsError>`](crate::error::GetDeploymentsError)
    pub fn get_deployments(&self) -> fluent_builders::GetDeployments {
        fluent_builders::GetDeployments::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`GetDeviceRegistration`](crate::client::fluent_builders::GetDeviceRegistration) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`device_name(impl Into<String>)`](crate::client::fluent_builders::GetDeviceRegistration::device_name) / [`set_device_name(Option<String>)`](crate::client::fluent_builders::GetDeviceRegistration::set_device_name): <p>The unique name of the device you want to get the registration status from.</p>
    ///   - [`device_fleet_name(impl Into<String>)`](crate::client::fluent_builders::GetDeviceRegistration::device_fleet_name) / [`set_device_fleet_name(Option<String>)`](crate::client::fluent_builders::GetDeviceRegistration::set_device_fleet_name): <p>The name of the fleet that the device belongs to.</p>
    /// - On success, responds with [`GetDeviceRegistrationOutput`](crate::output::GetDeviceRegistrationOutput) with field(s):
    ///   - [`device_registration(Option<String>)`](crate::output::GetDeviceRegistrationOutput::device_registration): <p>Describes if the device is currently registered with SageMaker Edge Manager.</p>
    ///   - [`cache_ttl(Option<String>)`](crate::output::GetDeviceRegistrationOutput::cache_ttl): <p>The amount of time, in seconds, that the registration status is stored on the device’s cache before it is refreshed.</p>
    /// - On failure, responds with [`SdkError<GetDeviceRegistrationError>`](crate::error::GetDeviceRegistrationError)
    pub fn get_device_registration(&self) -> fluent_builders::GetDeviceRegistration {
        fluent_builders::GetDeviceRegistration::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`SendHeartbeat`](crate::client::fluent_builders::SendHeartbeat) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_metrics(Vec<EdgeMetric>)`](crate::client::fluent_builders::SendHeartbeat::agent_metrics) / [`set_agent_metrics(Option<Vec<EdgeMetric>>)`](crate::client::fluent_builders::SendHeartbeat::set_agent_metrics): <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
    ///   - [`models(Vec<Model>)`](crate::client::fluent_builders::SendHeartbeat::models) / [`set_models(Option<Vec<Model>>)`](crate::client::fluent_builders::SendHeartbeat::set_models): <p>Returns a list of models deployed on the the device.</p>
    ///   - [`agent_version(impl Into<String>)`](crate::client::fluent_builders::SendHeartbeat::agent_version) / [`set_agent_version(Option<String>)`](crate::client::fluent_builders::SendHeartbeat::set_agent_version): <p>Returns the version of the agent.</p>
    ///   - [`device_name(impl Into<String>)`](crate::client::fluent_builders::SendHeartbeat::device_name) / [`set_device_name(Option<String>)`](crate::client::fluent_builders::SendHeartbeat::set_device_name): <p>The unique name of the device.</p>
    ///   - [`device_fleet_name(impl Into<String>)`](crate::client::fluent_builders::SendHeartbeat::device_fleet_name) / [`set_device_fleet_name(Option<String>)`](crate::client::fluent_builders::SendHeartbeat::set_device_fleet_name): <p>The name of the fleet that the device belongs to.</p>
    ///   - [`deployment_result(DeploymentResult)`](crate::client::fluent_builders::SendHeartbeat::deployment_result) / [`set_deployment_result(Option<DeploymentResult>)`](crate::client::fluent_builders::SendHeartbeat::set_deployment_result): <p>Returns the result of a deployment on the device.</p>
    /// - On success, responds with [`SendHeartbeatOutput`](crate::output::SendHeartbeatOutput)

    /// - On failure, responds with [`SdkError<SendHeartbeatError>`](crate::error::SendHeartbeatError)
    pub fn send_heartbeat(&self) -> fluent_builders::SendHeartbeat {
        fluent_builders::SendHeartbeat::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetDeployments`.
    ///
    /// <p>Use to get the active deployments from a device.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetDeployments {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_deployments_input::Builder,
    }
    impl GetDeployments {
        /// Creates a new `GetDeployments`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::GetDeployments,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetDeploymentsError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDeploymentsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetDeploymentsError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&self.handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            self.handle.client.call(op).await
        }
        /// <p>The unique name of the device you want to get the configuration of active deployments from.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_name(input.into());
            self
        }
        /// <p>The unique name of the device you want to get the configuration of active deployments from.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_name(input);
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_fleet_name(input.into());
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_fleet_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetDeviceRegistration`.
    ///
    /// <p>Use to check if a device is registered with SageMaker Edge Manager.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetDeviceRegistration {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_device_registration_input::Builder,
    }
    impl GetDeviceRegistration {
        /// Creates a new `GetDeviceRegistration`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::GetDeviceRegistration,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetDeviceRegistrationError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDeviceRegistrationOutput,
            aws_smithy_http::result::SdkError<crate::error::GetDeviceRegistrationError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&self.handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            self.handle.client.call(op).await
        }
        /// <p>The unique name of the device you want to get the registration status from.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_name(input.into());
            self
        }
        /// <p>The unique name of the device you want to get the registration status from.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_name(input);
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_fleet_name(input.into());
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_fleet_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendHeartbeat`.
    ///
    /// <p>Use to get the current status of devices registered on SageMaker Edge Manager.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendHeartbeat {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::send_heartbeat_input::Builder,
    }
    impl SendHeartbeat {
        /// Creates a new `SendHeartbeat`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::SendHeartbeat,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::SendHeartbeatError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendHeartbeatOutput,
            aws_smithy_http::result::SdkError<crate::error::SendHeartbeatError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&self.handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            self.handle.client.call(op).await
        }
        /// Appends an item to `AgentMetrics`.
        ///
        /// To override the contents of this collection use [`set_agent_metrics`](Self::set_agent_metrics).
        ///
        /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
        pub fn agent_metrics(mut self, input: crate::model::EdgeMetric) -> Self {
            self.inner = self.inner.agent_metrics(input);
            self
        }
        /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
        pub fn set_agent_metrics(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
        ) -> Self {
            self.inner = self.inner.set_agent_metrics(input);
            self
        }
        /// Appends an item to `Models`.
        ///
        /// To override the contents of this collection use [`set_models`](Self::set_models).
        ///
        /// <p>Returns a list of models deployed on the the device.</p>
        pub fn models(mut self, input: crate::model::Model) -> Self {
            self.inner = self.inner.models(input);
            self
        }
        /// <p>Returns a list of models deployed on the the device.</p>
        pub fn set_models(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Model>>,
        ) -> Self {
            self.inner = self.inner.set_models(input);
            self
        }
        /// <p>Returns the version of the agent.</p>
        pub fn agent_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.agent_version(input.into());
            self
        }
        /// <p>Returns the version of the agent.</p>
        pub fn set_agent_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_agent_version(input);
            self
        }
        /// <p>The unique name of the device.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_name(input.into());
            self
        }
        /// <p>The unique name of the device.</p>
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_name(input);
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_fleet_name(input.into());
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_fleet_name(input);
            self
        }
        /// <p>Returns the result of a deployment on the device.</p>
        pub fn deployment_result(mut self, input: crate::model::DeploymentResult) -> Self {
            self.inner = self.inner.deployment_result(input);
            self
        }
        /// <p>Returns the result of a deployment on the device.</p>
        pub fn set_deployment_result(
            mut self,
            input: std::option::Option<crate::model::DeploymentResult>,
        ) -> Self {
            self.inner = self.inner.set_deployment_result(input);
            self
        }
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf
            .timeout_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
        }

        let connector = conf.http_connector().and_then(|c| {
            let timeout_config = conf
                .timeout_config()
                .cloned()
                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
            let connector_settings =
                aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                    &timeout_config,
                );
            c.connector(&connector_settings, conf.sleep_impl())
        });

        let builder = aws_smithy_client::Builder::new();

        let builder = match connector {
            // Use provided connector
            Some(c) => builder.connector(c),
            None => {
                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                {
                    // Use default connector based on enabled features
                    builder.dyn_https_connector(
                        aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                            &timeout_config,
                        ),
                    )
                }
                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                {
                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                }
            }
        };
        let mut builder = builder
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ))
            .retry_config(retry_config.into())
            .operation_timeout_config(timeout_config.into());
        builder.set_sleep_impl(sleep_impl);
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
