// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateConnectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.provider_type {
        object.key("ProviderType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.connection_name {
        object.key("ConnectionName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.host_arn {
        object.key("HostArn").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_host_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateHostInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.provider_type {
        object.key("ProviderType").string(var_9.as_str());
    }
    if let Some(var_10) = &input.provider_endpoint {
        object.key("ProviderEndpoint").string(var_10.as_str());
    }
    if let Some(var_11) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("VpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_configuration(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("Tags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteConnectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.connection_arn {
        object.key("ConnectionArn").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_host_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteHostInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.host_arn {
        object.key("HostArn").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_connection_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetConnectionInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.connection_arn {
        object.key("ConnectionArn").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_host_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetHostInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.host_arn {
        object.key("HostArn").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_connections_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListConnectionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.provider_type_filter {
        object.key("ProviderTypeFilter").string(var_21.as_str());
    }
    if let Some(var_22) = &input.host_arn_filter {
        object.key("HostArnFilter").string(var_22.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_hosts_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListHostsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_24) = &input.next_token {
        object.key("NextToken").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.resource_arn {
        object.key("ResourceArn").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.resource_arn {
        object.key("ResourceArn").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("Tags").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.resource_arn {
        object.key("ResourceArn").string(var_31.as_str());
    }
    if let Some(var_32) = &input.tag_keys {
        let mut array_33 = object.key("TagKeys").start_array();
        for item_34 in var_32 {
            {
                array_33.value().string(item_34.as_str());
            }
        }
        array_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_host_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateHostInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.host_arn {
        object.key("HostArn").string(var_35.as_str());
    }
    if let Some(var_36) = &input.provider_endpoint {
        object.key("ProviderEndpoint").string(var_36.as_str());
    }
    if let Some(var_37) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_38 = object.key("VpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_configuration(&mut object_38, var_37)?;
        object_38.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.key {
        object.key("Key").string(var_39.as_str());
    }
    if let Some(var_40) = &input.value {
        object.key("Value").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.vpc_id {
        object.key("VpcId").string(var_41.as_str());
    }
    if let Some(var_42) = &input.subnet_ids {
        let mut array_43 = object.key("SubnetIds").start_array();
        for item_44 in var_42 {
            {
                array_43.value().string(item_44.as_str());
            }
        }
        array_43.finish();
    }
    if let Some(var_45) = &input.security_group_ids {
        let mut array_46 = object.key("SecurityGroupIds").start_array();
        for item_47 in var_45 {
            {
                array_46.value().string(item_47.as_str());
            }
        }
        array_46.finish();
    }
    if let Some(var_48) = &input.tls_certificate {
        object.key("TlsCertificate").string(var_48.as_str());
    }
    Ok(())
}
