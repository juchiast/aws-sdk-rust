// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_accept_eulas(
    input: &crate::input::AcceptEulasInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.client_token {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_launch_profile(
    input: &crate::input::CreateLaunchProfileInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_3) = &input.client_token {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_streaming_image(
    input: &crate::input::CreateStreamingImageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_5) = &input.client_token {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_streaming_session(
    input: &crate::input::CreateStreamingSessionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_7) = &input.client_token {
        let formatted_8 = inner_7.as_str();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_streaming_session_stream(
    input: &crate::input::CreateStreamingSessionStreamInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_9) = &input.client_token {
        let formatted_10 = inner_9.as_str();
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_studio(
    input: &crate::input::CreateStudioInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_11) = &input.client_token {
        let formatted_12 = inner_11.as_str();
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_studio_component(
    input: &crate::input::CreateStudioComponentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_13) = &input.client_token {
        let formatted_14 = inner_13.as_str();
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_launch_profile(
    input: &crate::input::DeleteLaunchProfileInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_15) = &input.client_token {
        let formatted_16 = inner_15.as_str();
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_launch_profile_member(
    input: &crate::input::DeleteLaunchProfileMemberInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_17) = &input.client_token {
        let formatted_18 = inner_17.as_str();
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_streaming_image(
    input: &crate::input::DeleteStreamingImageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_19) = &input.client_token {
        let formatted_20 = inner_19.as_str();
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_streaming_session(
    input: &crate::input::DeleteStreamingSessionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_21) = &input.client_token {
        let formatted_22 = inner_21.as_str();
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_studio(
    input: &crate::input::DeleteStudioInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_23) = &input.client_token {
        let formatted_24 = inner_23.as_str();
        if !formatted_24.is_empty() {
            let header_value = formatted_24;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_studio_component(
    input: &crate::input::DeleteStudioComponentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_25) = &input.client_token {
        let formatted_26 = inner_25.as_str();
        if !formatted_26.is_empty() {
            let header_value = formatted_26;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_delete_studio_member(
    input: &crate::input::DeleteStudioMemberInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_27) = &input.client_token {
        let formatted_28 = inner_27.as_str();
        if !formatted_28.is_empty() {
            let header_value = formatted_28;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_launch_profile_members(
    input: &crate::input::PutLaunchProfileMembersInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_29) = &input.client_token {
        let formatted_30 = inner_29.as_str();
        if !formatted_30.is_empty() {
            let header_value = formatted_30;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_put_studio_members(
    input: &crate::input::PutStudioMembersInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_31) = &input.client_token {
        let formatted_32 = inner_31.as_str();
        if !formatted_32.is_empty() {
            let header_value = formatted_32;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_start_streaming_session(
    input: &crate::input::StartStreamingSessionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_33) = &input.client_token {
        let formatted_34 = inner_33.as_str();
        if !formatted_34.is_empty() {
            let header_value = formatted_34;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_start_studio_sso_configuration_repair(
    input: &crate::input::StartStudioSsoConfigurationRepairInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_35) = &input.client_token {
        let formatted_36 = inner_35.as_str();
        if !formatted_36.is_empty() {
            let header_value = formatted_36;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_stop_streaming_session(
    input: &crate::input::StopStreamingSessionInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_37) = &input.client_token {
        let formatted_38 = inner_37.as_str();
        if !formatted_38.is_empty() {
            let header_value = formatted_38;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_launch_profile(
    input: &crate::input::UpdateLaunchProfileInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_39) = &input.client_token {
        let formatted_40 = inner_39.as_str();
        if !formatted_40.is_empty() {
            let header_value = formatted_40;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_launch_profile_member(
    input: &crate::input::UpdateLaunchProfileMemberInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_41) = &input.client_token {
        let formatted_42 = inner_41.as_str();
        if !formatted_42.is_empty() {
            let header_value = formatted_42;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_streaming_image(
    input: &crate::input::UpdateStreamingImageInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_43) = &input.client_token {
        let formatted_44 = inner_43.as_str();
        if !formatted_44.is_empty() {
            let header_value = formatted_44;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_studio(
    input: &crate::input::UpdateStudioInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_45) = &input.client_token {
        let formatted_46 = inner_45.as_str();
        if !formatted_46.is_empty() {
            let header_value = formatted_46;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_update_studio_component(
    input: &crate::input::UpdateStudioComponentInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_47) = &input.client_token {
        let formatted_48 = inner_47.as_str();
        if !formatted_48.is_empty() {
            let header_value = formatted_48;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "client_token",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("X-Amz-Client-Token", header_value);
        }
    }
    Ok(builder)
}
