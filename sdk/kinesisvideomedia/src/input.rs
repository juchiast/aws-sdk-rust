// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`GetMediaInput`](crate::input::GetMediaInput).
pub mod get_media_input {

    /// A builder for [`GetMediaInput`](crate::input::GetMediaInput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) stream_name: std::option::Option<std::string::String>,
        pub(crate) stream_arn: std::option::Option<std::string::String>,
        pub(crate) start_selector: std::option::Option<crate::model::StartSelector>,
    }
    impl Builder {
        /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
        pub fn stream_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_name = Some(input.into());
            self
        }
        /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
        pub fn set_stream_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_name = input;
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.stream_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.stream_arn = input;
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn start_selector(mut self, input: crate::model::StartSelector) -> Self {
            self.start_selector = Some(input);
            self
        }
        /// <p>Identifies the starting chunk to get from the specified stream. </p>
        pub fn set_start_selector(
            mut self,
            input: std::option::Option<crate::model::StartSelector>,
        ) -> Self {
            self.start_selector = input;
            self
        }
        /// Consumes the builder and constructs a [`GetMediaInput`](crate::input::GetMediaInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetMediaInput, aws_smithy_http::operation::error::BuildError>
        {
            Ok(crate::input::GetMediaInput {
                stream_name: self.stream_name,
                stream_arn: self.stream_arn,
                start_selector: self.start_selector,
            })
        }
    }
}
impl GetMediaInput {
    /// Consumes the builder and constructs an Operation<[`GetMedia`](crate::operation::GetMedia)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetMedia,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetMediaInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/getMedia").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetMediaInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_get_media(&self)?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::GetMedia::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "GetMedia",
                    "kinesisvideomedia",
                ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetMediaInput`](crate::input::GetMediaInput).
    pub fn builder() -> crate::input::get_media_input::Builder {
        crate::input::get_media_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetMediaInput {
    /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
    #[doc(hidden)]
    pub stream_name: std::option::Option<std::string::String>,
    /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    #[doc(hidden)]
    pub stream_arn: std::option::Option<std::string::String>,
    /// <p>Identifies the starting chunk to get from the specified stream. </p>
    #[doc(hidden)]
    pub start_selector: std::option::Option<crate::model::StartSelector>,
}
impl GetMediaInput {
    /// <p>The Kinesis video stream name from where you want to get the media content. If you don't specify the <code>streamName</code>, you must specify the <code>streamARN</code>.</p>
    pub fn stream_name(&self) -> std::option::Option<&str> {
        self.stream_name.as_deref()
    }
    /// <p>The ARN of the stream from where you want to get the media content. If you don't specify the <code>streamARN</code>, you must specify the <code>streamName</code>.</p>
    pub fn stream_arn(&self) -> std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>Identifies the starting chunk to get from the specified stream. </p>
    pub fn start_selector(&self) -> std::option::Option<&crate::model::StartSelector> {
        self.start_selector.as_ref()
    }
}
