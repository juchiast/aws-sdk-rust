// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchGetChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_channel`](crate::client::Client::batch_get_channel).
///
/// See [`crate::client::fluent_builders::BatchGetChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetChannel {
    _private: (),
}
impl BatchGetChannel {
    /// Creates a new builder-style object to manufacture [`BatchGetChannelInput`](crate::input::BatchGetChannelInput).
    pub fn builder() -> crate::input::batch_get_channel_input::Builder {
        crate::input::batch_get_channel_input::Builder::default()
    }
    /// Creates a new `BatchGetChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetChannel {
    type Output = std::result::Result<
        crate::output::BatchGetChannelOutput,
        crate::error::BatchGetChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_channel_error(response)
        } else {
            crate::operation_deser::parse_batch_get_channel_response(response)
        }
    }
}

/// Operation shape for `BatchGetStreamKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_stream_key`](crate::client::Client::batch_get_stream_key).
///
/// See [`crate::client::fluent_builders::BatchGetStreamKey`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetStreamKey {
    _private: (),
}
impl BatchGetStreamKey {
    /// Creates a new builder-style object to manufacture [`BatchGetStreamKeyInput`](crate::input::BatchGetStreamKeyInput).
    pub fn builder() -> crate::input::batch_get_stream_key_input::Builder {
        crate::input::batch_get_stream_key_input::Builder::default()
    }
    /// Creates a new `BatchGetStreamKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetStreamKey {
    type Output = std::result::Result<
        crate::output::BatchGetStreamKeyOutput,
        crate::error::BatchGetStreamKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_stream_key_error(response)
        } else {
            crate::operation_deser::parse_batch_get_stream_key_response(response)
        }
    }
}

/// Operation shape for `CreateChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_channel`](crate::client::Client::create_channel).
///
/// See [`crate::client::fluent_builders::CreateChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateChannel {
    _private: (),
}
impl CreateChannel {
    /// Creates a new builder-style object to manufacture [`CreateChannelInput`](crate::input::CreateChannelInput).
    pub fn builder() -> crate::input::create_channel_input::Builder {
        crate::input::create_channel_input::Builder::default()
    }
    /// Creates a new `CreateChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateChannel {
    type Output =
        std::result::Result<crate::output::CreateChannelOutput, crate::error::CreateChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_channel_error(response)
        } else {
            crate::operation_deser::parse_create_channel_response(response)
        }
    }
}

/// Operation shape for `CreateRecordingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_recording_configuration`](crate::client::Client::create_recording_configuration).
///
/// See [`crate::client::fluent_builders::CreateRecordingConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRecordingConfiguration {
    _private: (),
}
impl CreateRecordingConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateRecordingConfigurationInput`](crate::input::CreateRecordingConfigurationInput).
    pub fn builder() -> crate::input::create_recording_configuration_input::Builder {
        crate::input::create_recording_configuration_input::Builder::default()
    }
    /// Creates a new `CreateRecordingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRecordingConfiguration {
    type Output = std::result::Result<
        crate::output::CreateRecordingConfigurationOutput,
        crate::error::CreateRecordingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_recording_configuration_error(response)
        } else {
            crate::operation_deser::parse_create_recording_configuration_response(response)
        }
    }
}

/// Operation shape for `CreateStreamKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_stream_key`](crate::client::Client::create_stream_key).
///
/// See [`crate::client::fluent_builders::CreateStreamKey`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateStreamKey {
    _private: (),
}
impl CreateStreamKey {
    /// Creates a new builder-style object to manufacture [`CreateStreamKeyInput`](crate::input::CreateStreamKeyInput).
    pub fn builder() -> crate::input::create_stream_key_input::Builder {
        crate::input::create_stream_key_input::Builder::default()
    }
    /// Creates a new `CreateStreamKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateStreamKey {
    type Output = std::result::Result<
        crate::output::CreateStreamKeyOutput,
        crate::error::CreateStreamKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_stream_key_error(response)
        } else {
            crate::operation_deser::parse_create_stream_key_response(response)
        }
    }
}

/// Operation shape for `DeleteChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_channel`](crate::client::Client::delete_channel).
///
/// See [`crate::client::fluent_builders::DeleteChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteChannel {
    _private: (),
}
impl DeleteChannel {
    /// Creates a new builder-style object to manufacture [`DeleteChannelInput`](crate::input::DeleteChannelInput).
    pub fn builder() -> crate::input::delete_channel_input::Builder {
        crate::input::delete_channel_input::Builder::default()
    }
    /// Creates a new `DeleteChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteChannel {
    type Output =
        std::result::Result<crate::output::DeleteChannelOutput, crate::error::DeleteChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_channel_error(response)
        } else {
            crate::operation_deser::parse_delete_channel_response(response)
        }
    }
}

/// Operation shape for `DeletePlaybackKeyPair`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_playback_key_pair`](crate::client::Client::delete_playback_key_pair).
///
/// See [`crate::client::fluent_builders::DeletePlaybackKeyPair`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeletePlaybackKeyPair {
    _private: (),
}
impl DeletePlaybackKeyPair {
    /// Creates a new builder-style object to manufacture [`DeletePlaybackKeyPairInput`](crate::input::DeletePlaybackKeyPairInput).
    pub fn builder() -> crate::input::delete_playback_key_pair_input::Builder {
        crate::input::delete_playback_key_pair_input::Builder::default()
    }
    /// Creates a new `DeletePlaybackKeyPair` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePlaybackKeyPair {
    type Output = std::result::Result<
        crate::output::DeletePlaybackKeyPairOutput,
        crate::error::DeletePlaybackKeyPairError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_playback_key_pair_error(response)
        } else {
            crate::operation_deser::parse_delete_playback_key_pair_response(response)
        }
    }
}

/// Operation shape for `DeleteRecordingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_recording_configuration`](crate::client::Client::delete_recording_configuration).
///
/// See [`crate::client::fluent_builders::DeleteRecordingConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRecordingConfiguration {
    _private: (),
}
impl DeleteRecordingConfiguration {
    /// Creates a new builder-style object to manufacture [`DeleteRecordingConfigurationInput`](crate::input::DeleteRecordingConfigurationInput).
    pub fn builder() -> crate::input::delete_recording_configuration_input::Builder {
        crate::input::delete_recording_configuration_input::Builder::default()
    }
    /// Creates a new `DeleteRecordingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRecordingConfiguration {
    type Output = std::result::Result<
        crate::output::DeleteRecordingConfigurationOutput,
        crate::error::DeleteRecordingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_recording_configuration_error(response)
        } else {
            crate::operation_deser::parse_delete_recording_configuration_response(response)
        }
    }
}

/// Operation shape for `DeleteStreamKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_stream_key`](crate::client::Client::delete_stream_key).
///
/// See [`crate::client::fluent_builders::DeleteStreamKey`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteStreamKey {
    _private: (),
}
impl DeleteStreamKey {
    /// Creates a new builder-style object to manufacture [`DeleteStreamKeyInput`](crate::input::DeleteStreamKeyInput).
    pub fn builder() -> crate::input::delete_stream_key_input::Builder {
        crate::input::delete_stream_key_input::Builder::default()
    }
    /// Creates a new `DeleteStreamKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteStreamKey {
    type Output = std::result::Result<
        crate::output::DeleteStreamKeyOutput,
        crate::error::DeleteStreamKeyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_stream_key_error(response)
        } else {
            crate::operation_deser::parse_delete_stream_key_response(response)
        }
    }
}

/// Operation shape for `GetChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_channel`](crate::client::Client::get_channel).
///
/// See [`crate::client::fluent_builders::GetChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetChannel {
    _private: (),
}
impl GetChannel {
    /// Creates a new builder-style object to manufacture [`GetChannelInput`](crate::input::GetChannelInput).
    pub fn builder() -> crate::input::get_channel_input::Builder {
        crate::input::get_channel_input::Builder::default()
    }
    /// Creates a new `GetChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetChannel {
    type Output =
        std::result::Result<crate::output::GetChannelOutput, crate::error::GetChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_channel_error(response)
        } else {
            crate::operation_deser::parse_get_channel_response(response)
        }
    }
}

/// Operation shape for `GetPlaybackKeyPair`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_playback_key_pair`](crate::client::Client::get_playback_key_pair).
///
/// See [`crate::client::fluent_builders::GetPlaybackKeyPair`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPlaybackKeyPair {
    _private: (),
}
impl GetPlaybackKeyPair {
    /// Creates a new builder-style object to manufacture [`GetPlaybackKeyPairInput`](crate::input::GetPlaybackKeyPairInput).
    pub fn builder() -> crate::input::get_playback_key_pair_input::Builder {
        crate::input::get_playback_key_pair_input::Builder::default()
    }
    /// Creates a new `GetPlaybackKeyPair` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPlaybackKeyPair {
    type Output = std::result::Result<
        crate::output::GetPlaybackKeyPairOutput,
        crate::error::GetPlaybackKeyPairError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_playback_key_pair_error(response)
        } else {
            crate::operation_deser::parse_get_playback_key_pair_response(response)
        }
    }
}

/// Operation shape for `GetRecordingConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_recording_configuration`](crate::client::Client::get_recording_configuration).
///
/// See [`crate::client::fluent_builders::GetRecordingConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecordingConfiguration {
    _private: (),
}
impl GetRecordingConfiguration {
    /// Creates a new builder-style object to manufacture [`GetRecordingConfigurationInput`](crate::input::GetRecordingConfigurationInput).
    pub fn builder() -> crate::input::get_recording_configuration_input::Builder {
        crate::input::get_recording_configuration_input::Builder::default()
    }
    /// Creates a new `GetRecordingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecordingConfiguration {
    type Output = std::result::Result<
        crate::output::GetRecordingConfigurationOutput,
        crate::error::GetRecordingConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_recording_configuration_error(response)
        } else {
            crate::operation_deser::parse_get_recording_configuration_response(response)
        }
    }
}

/// Operation shape for `GetStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_stream`](crate::client::Client::get_stream).
///
/// See [`crate::client::fluent_builders::GetStream`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetStream {
    _private: (),
}
impl GetStream {
    /// Creates a new builder-style object to manufacture [`GetStreamInput`](crate::input::GetStreamInput).
    pub fn builder() -> crate::input::get_stream_input::Builder {
        crate::input::get_stream_input::Builder::default()
    }
    /// Creates a new `GetStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetStream {
    type Output = std::result::Result<crate::output::GetStreamOutput, crate::error::GetStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_stream_error(response)
        } else {
            crate::operation_deser::parse_get_stream_response(response)
        }
    }
}

/// Operation shape for `GetStreamKey`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_stream_key`](crate::client::Client::get_stream_key).
///
/// See [`crate::client::fluent_builders::GetStreamKey`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetStreamKey {
    _private: (),
}
impl GetStreamKey {
    /// Creates a new builder-style object to manufacture [`GetStreamKeyInput`](crate::input::GetStreamKeyInput).
    pub fn builder() -> crate::input::get_stream_key_input::Builder {
        crate::input::get_stream_key_input::Builder::default()
    }
    /// Creates a new `GetStreamKey` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetStreamKey {
    type Output =
        std::result::Result<crate::output::GetStreamKeyOutput, crate::error::GetStreamKeyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_stream_key_error(response)
        } else {
            crate::operation_deser::parse_get_stream_key_response(response)
        }
    }
}

/// Operation shape for `GetStreamSession`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_stream_session`](crate::client::Client::get_stream_session).
///
/// See [`crate::client::fluent_builders::GetStreamSession`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetStreamSession {
    _private: (),
}
impl GetStreamSession {
    /// Creates a new builder-style object to manufacture [`GetStreamSessionInput`](crate::input::GetStreamSessionInput).
    pub fn builder() -> crate::input::get_stream_session_input::Builder {
        crate::input::get_stream_session_input::Builder::default()
    }
    /// Creates a new `GetStreamSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetStreamSession {
    type Output = std::result::Result<
        crate::output::GetStreamSessionOutput,
        crate::error::GetStreamSessionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_stream_session_error(response)
        } else {
            crate::operation_deser::parse_get_stream_session_response(response)
        }
    }
}

/// Operation shape for `ImportPlaybackKeyPair`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`import_playback_key_pair`](crate::client::Client::import_playback_key_pair).
///
/// See [`crate::client::fluent_builders::ImportPlaybackKeyPair`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportPlaybackKeyPair {
    _private: (),
}
impl ImportPlaybackKeyPair {
    /// Creates a new builder-style object to manufacture [`ImportPlaybackKeyPairInput`](crate::input::ImportPlaybackKeyPairInput).
    pub fn builder() -> crate::input::import_playback_key_pair_input::Builder {
        crate::input::import_playback_key_pair_input::Builder::default()
    }
    /// Creates a new `ImportPlaybackKeyPair` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportPlaybackKeyPair {
    type Output = std::result::Result<
        crate::output::ImportPlaybackKeyPairOutput,
        crate::error::ImportPlaybackKeyPairError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_import_playback_key_pair_error(response)
        } else {
            crate::operation_deser::parse_import_playback_key_pair_response(response)
        }
    }
}

/// Operation shape for `ListChannels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_channels`](crate::client::Client::list_channels).
///
/// See [`crate::client::fluent_builders::ListChannels`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListChannels {
    _private: (),
}
impl ListChannels {
    /// Creates a new builder-style object to manufacture [`ListChannelsInput`](crate::input::ListChannelsInput).
    pub fn builder() -> crate::input::list_channels_input::Builder {
        crate::input::list_channels_input::Builder::default()
    }
    /// Creates a new `ListChannels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListChannels {
    type Output =
        std::result::Result<crate::output::ListChannelsOutput, crate::error::ListChannelsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_channels_error(response)
        } else {
            crate::operation_deser::parse_list_channels_response(response)
        }
    }
}

/// Operation shape for `ListPlaybackKeyPairs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_playback_key_pairs`](crate::client::Client::list_playback_key_pairs).
///
/// See [`crate::client::fluent_builders::ListPlaybackKeyPairs`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPlaybackKeyPairs {
    _private: (),
}
impl ListPlaybackKeyPairs {
    /// Creates a new builder-style object to manufacture [`ListPlaybackKeyPairsInput`](crate::input::ListPlaybackKeyPairsInput).
    pub fn builder() -> crate::input::list_playback_key_pairs_input::Builder {
        crate::input::list_playback_key_pairs_input::Builder::default()
    }
    /// Creates a new `ListPlaybackKeyPairs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPlaybackKeyPairs {
    type Output = std::result::Result<
        crate::output::ListPlaybackKeyPairsOutput,
        crate::error::ListPlaybackKeyPairsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_playback_key_pairs_error(response)
        } else {
            crate::operation_deser::parse_list_playback_key_pairs_response(response)
        }
    }
}

/// Operation shape for `ListRecordingConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_recording_configurations`](crate::client::Client::list_recording_configurations).
///
/// See [`crate::client::fluent_builders::ListRecordingConfigurations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRecordingConfigurations {
    _private: (),
}
impl ListRecordingConfigurations {
    /// Creates a new builder-style object to manufacture [`ListRecordingConfigurationsInput`](crate::input::ListRecordingConfigurationsInput).
    pub fn builder() -> crate::input::list_recording_configurations_input::Builder {
        crate::input::list_recording_configurations_input::Builder::default()
    }
    /// Creates a new `ListRecordingConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRecordingConfigurations {
    type Output = std::result::Result<
        crate::output::ListRecordingConfigurationsOutput,
        crate::error::ListRecordingConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_recording_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_recording_configurations_response(response)
        }
    }
}

/// Operation shape for `ListStreamKeys`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_stream_keys`](crate::client::Client::list_stream_keys).
///
/// See [`crate::client::fluent_builders::ListStreamKeys`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListStreamKeys {
    _private: (),
}
impl ListStreamKeys {
    /// Creates a new builder-style object to manufacture [`ListStreamKeysInput`](crate::input::ListStreamKeysInput).
    pub fn builder() -> crate::input::list_stream_keys_input::Builder {
        crate::input::list_stream_keys_input::Builder::default()
    }
    /// Creates a new `ListStreamKeys` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStreamKeys {
    type Output =
        std::result::Result<crate::output::ListStreamKeysOutput, crate::error::ListStreamKeysError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_stream_keys_error(response)
        } else {
            crate::operation_deser::parse_list_stream_keys_response(response)
        }
    }
}

/// Operation shape for `ListStreams`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_streams`](crate::client::Client::list_streams).
///
/// See [`crate::client::fluent_builders::ListStreams`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListStreams {
    _private: (),
}
impl ListStreams {
    /// Creates a new builder-style object to manufacture [`ListStreamsInput`](crate::input::ListStreamsInput).
    pub fn builder() -> crate::input::list_streams_input::Builder {
        crate::input::list_streams_input::Builder::default()
    }
    /// Creates a new `ListStreams` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStreams {
    type Output =
        std::result::Result<crate::output::ListStreamsOutput, crate::error::ListStreamsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_streams_error(response)
        } else {
            crate::operation_deser::parse_list_streams_response(response)
        }
    }
}

/// Operation shape for `ListStreamSessions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_stream_sessions`](crate::client::Client::list_stream_sessions).
///
/// See [`crate::client::fluent_builders::ListStreamSessions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListStreamSessions {
    _private: (),
}
impl ListStreamSessions {
    /// Creates a new builder-style object to manufacture [`ListStreamSessionsInput`](crate::input::ListStreamSessionsInput).
    pub fn builder() -> crate::input::list_stream_sessions_input::Builder {
        crate::input::list_stream_sessions_input::Builder::default()
    }
    /// Creates a new `ListStreamSessions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListStreamSessions {
    type Output = std::result::Result<
        crate::output::ListStreamSessionsOutput,
        crate::error::ListStreamSessionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_stream_sessions_error(response)
        } else {
            crate::operation_deser::parse_list_stream_sessions_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `PutMetadata`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_metadata`](crate::client::Client::put_metadata).
///
/// See [`crate::client::fluent_builders::PutMetadata`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutMetadata {
    _private: (),
}
impl PutMetadata {
    /// Creates a new builder-style object to manufacture [`PutMetadataInput`](crate::input::PutMetadataInput).
    pub fn builder() -> crate::input::put_metadata_input::Builder {
        crate::input::put_metadata_input::Builder::default()
    }
    /// Creates a new `PutMetadata` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutMetadata {
    type Output =
        std::result::Result<crate::output::PutMetadataOutput, crate::error::PutMetadataError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_metadata_error(response)
        } else {
            crate::operation_deser::parse_put_metadata_response(response)
        }
    }
}

/// Operation shape for `StopStream`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_stream`](crate::client::Client::stop_stream).
///
/// See [`crate::client::fluent_builders::StopStream`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopStream {
    _private: (),
}
impl StopStream {
    /// Creates a new builder-style object to manufacture [`StopStreamInput`](crate::input::StopStreamInput).
    pub fn builder() -> crate::input::stop_stream_input::Builder {
        crate::input::stop_stream_input::Builder::default()
    }
    /// Creates a new `StopStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopStream {
    type Output =
        std::result::Result<crate::output::StopStreamOutput, crate::error::StopStreamError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_stream_error(response)
        } else {
            crate::operation_deser::parse_stop_stream_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_channel`](crate::client::Client::update_channel).
///
/// See [`crate::client::fluent_builders::UpdateChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateChannel {
    _private: (),
}
impl UpdateChannel {
    /// Creates a new builder-style object to manufacture [`UpdateChannelInput`](crate::input::UpdateChannelInput).
    pub fn builder() -> crate::input::update_channel_input::Builder {
        crate::input::update_channel_input::Builder::default()
    }
    /// Creates a new `UpdateChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateChannel {
    type Output =
        std::result::Result<crate::output::UpdateChannelOutput, crate::error::UpdateChannelError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_channel_error(response)
        } else {
            crate::operation_deser::parse_update_channel_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
