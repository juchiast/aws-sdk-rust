// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_get_profile(
    input: &crate::input::GetProfileInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.accept {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "accept",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("Accept", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_post_agent_profile(
    input: &crate::input::PostAgentProfileInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_3) = &input.content_type {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "content_type",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("Content-Type", header_value);
        }
    }
    Ok(builder)
}

pub fn deser_payload_configure_agent_configure_agent_output_configuration(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::AgentConfiguration>,
    crate::error::ConfigureAgentError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_agent_configuration_payload(body)
                .map_err(crate::error::ConfigureAgentError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_create_profiling_group_create_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::CreateProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::CreateProfilingGroupError::unhandled)
        })
        .transpose()
}

pub fn deser_payload_describe_profiling_group_describe_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::DescribeProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::DescribeProfilingGroupError::unhandled)
        })
        .transpose()
}

pub(crate) fn deser_header_get_profile_get_profile_output_content_encoding(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Encoding").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_profile_get_profile_output_content_type(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Content-Type").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub fn deser_payload_get_profile_get_profile_output_profile(
    body: &[u8],
) -> std::result::Result<std::option::Option<aws_smithy_types::Blob>, crate::error::GetProfileError>
{
    (!body.is_empty())
        .then(|| Ok(aws_smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_payload_update_profiling_group_update_profiling_group_output_profiling_group(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::model::ProfilingGroupDescription>,
    crate::error::UpdateProfilingGroupError,
> {
    (!body.is_empty())
        .then(|| {
            crate::json_deser::deser_structure_crate_model_profiling_group_description_payload(body)
                .map_err(crate::error::UpdateProfilingGroupError::unhandled)
        })
        .transpose()
}
