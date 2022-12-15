// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AcceptPage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`accept_page`](crate::client::Client::accept_page).
///
/// See [`crate::client::fluent_builders::AcceptPage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AcceptPage {
    _private: (),
}
impl AcceptPage {
    /// Creates a new builder-style object to manufacture [`AcceptPageInput`](crate::input::AcceptPageInput).
    pub fn builder() -> crate::input::accept_page_input::Builder {
        crate::input::accept_page_input::Builder::default()
    }
    /// Creates a new `AcceptPage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AcceptPage {
    type Output =
        std::result::Result<crate::output::AcceptPageOutput, crate::error::AcceptPageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_accept_page_error(response)
        } else {
            crate::operation_deser::parse_accept_page_response(response)
        }
    }
}

/// Operation shape for `ActivateContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`activate_contact_channel`](crate::client::Client::activate_contact_channel).
///
/// See [`crate::client::fluent_builders::ActivateContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ActivateContactChannel {
    _private: (),
}
impl ActivateContactChannel {
    /// Creates a new builder-style object to manufacture [`ActivateContactChannelInput`](crate::input::ActivateContactChannelInput).
    pub fn builder() -> crate::input::activate_contact_channel_input::Builder {
        crate::input::activate_contact_channel_input::Builder::default()
    }
    /// Creates a new `ActivateContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ActivateContactChannel {
    type Output = std::result::Result<
        crate::output::ActivateContactChannelOutput,
        crate::error::ActivateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_activate_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_activate_contact_channel_response(response)
        }
    }
}

/// Operation shape for `CreateContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_contact`](crate::client::Client::create_contact).
///
/// See [`crate::client::fluent_builders::CreateContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateContact {
    _private: (),
}
impl CreateContact {
    /// Creates a new builder-style object to manufacture [`CreateContactInput`](crate::input::CreateContactInput).
    pub fn builder() -> crate::input::create_contact_input::Builder {
        crate::input::create_contact_input::Builder::default()
    }
    /// Creates a new `CreateContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateContact {
    type Output =
        std::result::Result<crate::output::CreateContactOutput, crate::error::CreateContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_contact_error(response)
        } else {
            crate::operation_deser::parse_create_contact_response(response)
        }
    }
}

/// Operation shape for `CreateContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_contact_channel`](crate::client::Client::create_contact_channel).
///
/// See [`crate::client::fluent_builders::CreateContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateContactChannel {
    _private: (),
}
impl CreateContactChannel {
    /// Creates a new builder-style object to manufacture [`CreateContactChannelInput`](crate::input::CreateContactChannelInput).
    pub fn builder() -> crate::input::create_contact_channel_input::Builder {
        crate::input::create_contact_channel_input::Builder::default()
    }
    /// Creates a new `CreateContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateContactChannel {
    type Output = std::result::Result<
        crate::output::CreateContactChannelOutput,
        crate::error::CreateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_create_contact_channel_response(response)
        }
    }
}

/// Operation shape for `DeactivateContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deactivate_contact_channel`](crate::client::Client::deactivate_contact_channel).
///
/// See [`crate::client::fluent_builders::DeactivateContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeactivateContactChannel {
    _private: (),
}
impl DeactivateContactChannel {
    /// Creates a new builder-style object to manufacture [`DeactivateContactChannelInput`](crate::input::DeactivateContactChannelInput).
    pub fn builder() -> crate::input::deactivate_contact_channel_input::Builder {
        crate::input::deactivate_contact_channel_input::Builder::default()
    }
    /// Creates a new `DeactivateContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeactivateContactChannel {
    type Output = std::result::Result<
        crate::output::DeactivateContactChannelOutput,
        crate::error::DeactivateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deactivate_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_deactivate_contact_channel_response(response)
        }
    }
}

/// Operation shape for `DeleteContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_contact`](crate::client::Client::delete_contact).
///
/// See [`crate::client::fluent_builders::DeleteContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteContact {
    _private: (),
}
impl DeleteContact {
    /// Creates a new builder-style object to manufacture [`DeleteContactInput`](crate::input::DeleteContactInput).
    pub fn builder() -> crate::input::delete_contact_input::Builder {
        crate::input::delete_contact_input::Builder::default()
    }
    /// Creates a new `DeleteContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteContact {
    type Output =
        std::result::Result<crate::output::DeleteContactOutput, crate::error::DeleteContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_contact_error(response)
        } else {
            crate::operation_deser::parse_delete_contact_response(response)
        }
    }
}

/// Operation shape for `DeleteContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_contact_channel`](crate::client::Client::delete_contact_channel).
///
/// See [`crate::client::fluent_builders::DeleteContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteContactChannel {
    _private: (),
}
impl DeleteContactChannel {
    /// Creates a new builder-style object to manufacture [`DeleteContactChannelInput`](crate::input::DeleteContactChannelInput).
    pub fn builder() -> crate::input::delete_contact_channel_input::Builder {
        crate::input::delete_contact_channel_input::Builder::default()
    }
    /// Creates a new `DeleteContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteContactChannel {
    type Output = std::result::Result<
        crate::output::DeleteContactChannelOutput,
        crate::error::DeleteContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_delete_contact_channel_response(response)
        }
    }
}

/// Operation shape for `DescribeEngagement`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_engagement`](crate::client::Client::describe_engagement).
///
/// See [`crate::client::fluent_builders::DescribeEngagement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEngagement {
    _private: (),
}
impl DescribeEngagement {
    /// Creates a new builder-style object to manufacture [`DescribeEngagementInput`](crate::input::DescribeEngagementInput).
    pub fn builder() -> crate::input::describe_engagement_input::Builder {
        crate::input::describe_engagement_input::Builder::default()
    }
    /// Creates a new `DescribeEngagement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEngagement {
    type Output = std::result::Result<
        crate::output::DescribeEngagementOutput,
        crate::error::DescribeEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_engagement_error(response)
        } else {
            crate::operation_deser::parse_describe_engagement_response(response)
        }
    }
}

/// Operation shape for `DescribePage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_page`](crate::client::Client::describe_page).
///
/// See [`crate::client::fluent_builders::DescribePage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribePage {
    _private: (),
}
impl DescribePage {
    /// Creates a new builder-style object to manufacture [`DescribePageInput`](crate::input::DescribePageInput).
    pub fn builder() -> crate::input::describe_page_input::Builder {
        crate::input::describe_page_input::Builder::default()
    }
    /// Creates a new `DescribePage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribePage {
    type Output =
        std::result::Result<crate::output::DescribePageOutput, crate::error::DescribePageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_page_error(response)
        } else {
            crate::operation_deser::parse_describe_page_response(response)
        }
    }
}

/// Operation shape for `GetContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_contact`](crate::client::Client::get_contact).
///
/// See [`crate::client::fluent_builders::GetContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetContact {
    _private: (),
}
impl GetContact {
    /// Creates a new builder-style object to manufacture [`GetContactInput`](crate::input::GetContactInput).
    pub fn builder() -> crate::input::get_contact_input::Builder {
        crate::input::get_contact_input::Builder::default()
    }
    /// Creates a new `GetContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetContact {
    type Output =
        std::result::Result<crate::output::GetContactOutput, crate::error::GetContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_error(response)
        } else {
            crate::operation_deser::parse_get_contact_response(response)
        }
    }
}

/// Operation shape for `GetContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_contact_channel`](crate::client::Client::get_contact_channel).
///
/// See [`crate::client::fluent_builders::GetContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetContactChannel {
    _private: (),
}
impl GetContactChannel {
    /// Creates a new builder-style object to manufacture [`GetContactChannelInput`](crate::input::GetContactChannelInput).
    pub fn builder() -> crate::input::get_contact_channel_input::Builder {
        crate::input::get_contact_channel_input::Builder::default()
    }
    /// Creates a new `GetContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetContactChannel {
    type Output = std::result::Result<
        crate::output::GetContactChannelOutput,
        crate::error::GetContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_get_contact_channel_response(response)
        }
    }
}

/// Operation shape for `GetContactPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_contact_policy`](crate::client::Client::get_contact_policy).
///
/// See [`crate::client::fluent_builders::GetContactPolicy`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetContactPolicy {
    _private: (),
}
impl GetContactPolicy {
    /// Creates a new builder-style object to manufacture [`GetContactPolicyInput`](crate::input::GetContactPolicyInput).
    pub fn builder() -> crate::input::get_contact_policy_input::Builder {
        crate::input::get_contact_policy_input::Builder::default()
    }
    /// Creates a new `GetContactPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetContactPolicy {
    type Output = std::result::Result<
        crate::output::GetContactPolicyOutput,
        crate::error::GetContactPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_contact_policy_error(response)
        } else {
            crate::operation_deser::parse_get_contact_policy_response(response)
        }
    }
}

/// Operation shape for `ListContactChannels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_contact_channels`](crate::client::Client::list_contact_channels).
///
/// See [`crate::client::fluent_builders::ListContactChannels`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListContactChannels {
    _private: (),
}
impl ListContactChannels {
    /// Creates a new builder-style object to manufacture [`ListContactChannelsInput`](crate::input::ListContactChannelsInput).
    pub fn builder() -> crate::input::list_contact_channels_input::Builder {
        crate::input::list_contact_channels_input::Builder::default()
    }
    /// Creates a new `ListContactChannels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListContactChannels {
    type Output = std::result::Result<
        crate::output::ListContactChannelsOutput,
        crate::error::ListContactChannelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_contact_channels_error(response)
        } else {
            crate::operation_deser::parse_list_contact_channels_response(response)
        }
    }
}

/// Operation shape for `ListContacts`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_contacts`](crate::client::Client::list_contacts).
///
/// See [`crate::client::fluent_builders::ListContacts`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListContacts {
    _private: (),
}
impl ListContacts {
    /// Creates a new builder-style object to manufacture [`ListContactsInput`](crate::input::ListContactsInput).
    pub fn builder() -> crate::input::list_contacts_input::Builder {
        crate::input::list_contacts_input::Builder::default()
    }
    /// Creates a new `ListContacts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListContacts {
    type Output =
        std::result::Result<crate::output::ListContactsOutput, crate::error::ListContactsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_contacts_error(response)
        } else {
            crate::operation_deser::parse_list_contacts_response(response)
        }
    }
}

/// Operation shape for `ListEngagements`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_engagements`](crate::client::Client::list_engagements).
///
/// See [`crate::client::fluent_builders::ListEngagements`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListEngagements {
    _private: (),
}
impl ListEngagements {
    /// Creates a new builder-style object to manufacture [`ListEngagementsInput`](crate::input::ListEngagementsInput).
    pub fn builder() -> crate::input::list_engagements_input::Builder {
        crate::input::list_engagements_input::Builder::default()
    }
    /// Creates a new `ListEngagements` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEngagements {
    type Output = std::result::Result<
        crate::output::ListEngagementsOutput,
        crate::error::ListEngagementsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_engagements_error(response)
        } else {
            crate::operation_deser::parse_list_engagements_response(response)
        }
    }
}

/// Operation shape for `ListPageReceipts`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_page_receipts`](crate::client::Client::list_page_receipts).
///
/// See [`crate::client::fluent_builders::ListPageReceipts`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPageReceipts {
    _private: (),
}
impl ListPageReceipts {
    /// Creates a new builder-style object to manufacture [`ListPageReceiptsInput`](crate::input::ListPageReceiptsInput).
    pub fn builder() -> crate::input::list_page_receipts_input::Builder {
        crate::input::list_page_receipts_input::Builder::default()
    }
    /// Creates a new `ListPageReceipts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPageReceipts {
    type Output = std::result::Result<
        crate::output::ListPageReceiptsOutput,
        crate::error::ListPageReceiptsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_page_receipts_error(response)
        } else {
            crate::operation_deser::parse_list_page_receipts_response(response)
        }
    }
}

/// Operation shape for `ListPagesByContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_pages_by_contact`](crate::client::Client::list_pages_by_contact).
///
/// See [`crate::client::fluent_builders::ListPagesByContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPagesByContact {
    _private: (),
}
impl ListPagesByContact {
    /// Creates a new builder-style object to manufacture [`ListPagesByContactInput`](crate::input::ListPagesByContactInput).
    pub fn builder() -> crate::input::list_pages_by_contact_input::Builder {
        crate::input::list_pages_by_contact_input::Builder::default()
    }
    /// Creates a new `ListPagesByContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPagesByContact {
    type Output = std::result::Result<
        crate::output::ListPagesByContactOutput,
        crate::error::ListPagesByContactError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pages_by_contact_error(response)
        } else {
            crate::operation_deser::parse_list_pages_by_contact_response(response)
        }
    }
}

/// Operation shape for `ListPagesByEngagement`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_pages_by_engagement`](crate::client::Client::list_pages_by_engagement).
///
/// See [`crate::client::fluent_builders::ListPagesByEngagement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPagesByEngagement {
    _private: (),
}
impl ListPagesByEngagement {
    /// Creates a new builder-style object to manufacture [`ListPagesByEngagementInput`](crate::input::ListPagesByEngagementInput).
    pub fn builder() -> crate::input::list_pages_by_engagement_input::Builder {
        crate::input::list_pages_by_engagement_input::Builder::default()
    }
    /// Creates a new `ListPagesByEngagement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPagesByEngagement {
    type Output = std::result::Result<
        crate::output::ListPagesByEngagementOutput,
        crate::error::ListPagesByEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pages_by_engagement_error(response)
        } else {
            crate::operation_deser::parse_list_pages_by_engagement_response(response)
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

/// Operation shape for `PutContactPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_contact_policy`](crate::client::Client::put_contact_policy).
///
/// See [`crate::client::fluent_builders::PutContactPolicy`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutContactPolicy {
    _private: (),
}
impl PutContactPolicy {
    /// Creates a new builder-style object to manufacture [`PutContactPolicyInput`](crate::input::PutContactPolicyInput).
    pub fn builder() -> crate::input::put_contact_policy_input::Builder {
        crate::input::put_contact_policy_input::Builder::default()
    }
    /// Creates a new `PutContactPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutContactPolicy {
    type Output = std::result::Result<
        crate::output::PutContactPolicyOutput,
        crate::error::PutContactPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_contact_policy_error(response)
        } else {
            crate::operation_deser::parse_put_contact_policy_response(response)
        }
    }
}

/// Operation shape for `SendActivationCode`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_activation_code`](crate::client::Client::send_activation_code).
///
/// See [`crate::client::fluent_builders::SendActivationCode`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SendActivationCode {
    _private: (),
}
impl SendActivationCode {
    /// Creates a new builder-style object to manufacture [`SendActivationCodeInput`](crate::input::SendActivationCodeInput).
    pub fn builder() -> crate::input::send_activation_code_input::Builder {
        crate::input::send_activation_code_input::Builder::default()
    }
    /// Creates a new `SendActivationCode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendActivationCode {
    type Output = std::result::Result<
        crate::output::SendActivationCodeOutput,
        crate::error::SendActivationCodeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_activation_code_error(response)
        } else {
            crate::operation_deser::parse_send_activation_code_response(response)
        }
    }
}

/// Operation shape for `StartEngagement`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_engagement`](crate::client::Client::start_engagement).
///
/// See [`crate::client::fluent_builders::StartEngagement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartEngagement {
    _private: (),
}
impl StartEngagement {
    /// Creates a new builder-style object to manufacture [`StartEngagementInput`](crate::input::StartEngagementInput).
    pub fn builder() -> crate::input::start_engagement_input::Builder {
        crate::input::start_engagement_input::Builder::default()
    }
    /// Creates a new `StartEngagement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartEngagement {
    type Output = std::result::Result<
        crate::output::StartEngagementOutput,
        crate::error::StartEngagementError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_engagement_error(response)
        } else {
            crate::operation_deser::parse_start_engagement_response(response)
        }
    }
}

/// Operation shape for `StopEngagement`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_engagement`](crate::client::Client::stop_engagement).
///
/// See [`crate::client::fluent_builders::StopEngagement`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopEngagement {
    _private: (),
}
impl StopEngagement {
    /// Creates a new builder-style object to manufacture [`StopEngagementInput`](crate::input::StopEngagementInput).
    pub fn builder() -> crate::input::stop_engagement_input::Builder {
        crate::input::stop_engagement_input::Builder::default()
    }
    /// Creates a new `StopEngagement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopEngagement {
    type Output =
        std::result::Result<crate::output::StopEngagementOutput, crate::error::StopEngagementError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_engagement_error(response)
        } else {
            crate::operation_deser::parse_stop_engagement_response(response)
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

/// Operation shape for `UpdateContact`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_contact`](crate::client::Client::update_contact).
///
/// See [`crate::client::fluent_builders::UpdateContact`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateContact {
    _private: (),
}
impl UpdateContact {
    /// Creates a new builder-style object to manufacture [`UpdateContactInput`](crate::input::UpdateContactInput).
    pub fn builder() -> crate::input::update_contact_input::Builder {
        crate::input::update_contact_input::Builder::default()
    }
    /// Creates a new `UpdateContact` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateContact {
    type Output =
        std::result::Result<crate::output::UpdateContactOutput, crate::error::UpdateContactError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_contact_error(response)
        } else {
            crate::operation_deser::parse_update_contact_response(response)
        }
    }
}

/// Operation shape for `UpdateContactChannel`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_contact_channel`](crate::client::Client::update_contact_channel).
///
/// See [`crate::client::fluent_builders::UpdateContactChannel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateContactChannel {
    _private: (),
}
impl UpdateContactChannel {
    /// Creates a new builder-style object to manufacture [`UpdateContactChannelInput`](crate::input::UpdateContactChannelInput).
    pub fn builder() -> crate::input::update_contact_channel_input::Builder {
        crate::input::update_contact_channel_input::Builder::default()
    }
    /// Creates a new `UpdateContactChannel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateContactChannel {
    type Output = std::result::Result<
        crate::output::UpdateContactChannelOutput,
        crate::error::UpdateContactChannelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_contact_channel_error(response)
        } else {
            crate::operation_deser::parse_update_contact_channel_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
