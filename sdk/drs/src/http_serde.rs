// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn deser_header_create_extended_source_server_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub(crate) fn deser_header_create_extended_source_server_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_create_replication_configuration_template_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub(crate) fn deser_header_create_replication_configuration_template_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_delete_job_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_3.len()
        )))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub(crate) fn deser_header_delete_job_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_delete_recovery_instance_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_4.len()
        )))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub(crate) fn deser_header_delete_recovery_instance_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_delete_replication_configuration_template_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_5 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_5.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_5.len()
        )))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub(crate) fn deser_header_delete_replication_configuration_template_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_delete_source_server_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_6 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_6.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_6.len()
        )))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub(crate) fn deser_header_delete_source_server_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_job_log_items_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_7 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_7.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_7.len()
        )))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub(crate) fn deser_header_describe_job_log_items_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_jobs_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_8 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_8.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_8.len()
        )))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub(crate) fn deser_header_describe_jobs_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_recovery_instances_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_9 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_9.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_9.len()
        )))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub(crate) fn deser_header_describe_recovery_instances_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_recovery_snapshots_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_10 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_10.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_10.len()
        )))
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}

pub(crate) fn deser_header_describe_recovery_snapshots_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_replication_configuration_templates_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_11 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_11.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_11.len()
        )))
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub(crate) fn deser_header_describe_replication_configuration_templates_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_describe_source_servers_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_12 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_12.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_12.len()
        )))
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub(crate) fn deser_header_describe_source_servers_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_disconnect_recovery_instance_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_13 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_13.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_13.len()
        )))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub(crate) fn deser_header_disconnect_recovery_instance_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_disconnect_source_server_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_14 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_14.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_14.len()
        )))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub(crate) fn deser_header_disconnect_source_server_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_failback_replication_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_15 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_15.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_15.len()
        )))
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub(crate) fn deser_header_get_failback_replication_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_launch_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_16 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_16.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_16.len()
        )))
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub(crate) fn deser_header_get_launch_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_get_replication_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_17 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_17.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_17.len()
        )))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub(crate) fn deser_header_get_replication_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_initialize_service_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_18 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_18.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_18.len()
        )))
    } else {
        let mut var_18 = var_18;
        Ok(var_18.pop())
    }
}

pub(crate) fn deser_header_initialize_service_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_list_extensible_source_servers_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_19 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_19.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_19.len()
        )))
    } else {
        let mut var_19 = var_19;
        Ok(var_19.pop())
    }
}

pub(crate) fn deser_header_list_extensible_source_servers_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_list_staging_accounts_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_20 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_20.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_20.len()
        )))
    } else {
        let mut var_20 = var_20;
        Ok(var_20.pop())
    }
}

pub(crate) fn deser_header_list_staging_accounts_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_list_tags_for_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_21 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_21.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_21.len()
        )))
    } else {
        let mut var_21 = var_21;
        Ok(var_21.pop())
    }
}

pub(crate) fn deser_header_list_tags_for_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_retry_data_replication_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_22 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_22.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_22.len()
        )))
    } else {
        let mut var_22 = var_22;
        Ok(var_22.pop())
    }
}

pub(crate) fn deser_header_retry_data_replication_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_start_failback_launch_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_23 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_23.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_23.len()
        )))
    } else {
        let mut var_23 = var_23;
        Ok(var_23.pop())
    }
}

pub(crate) fn deser_header_start_failback_launch_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_start_recovery_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_24 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_24.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_24.len()
        )))
    } else {
        let mut var_24 = var_24;
        Ok(var_24.pop())
    }
}

pub(crate) fn deser_header_start_recovery_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_stop_failback_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_25 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_25.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_25.len()
        )))
    } else {
        let mut var_25 = var_25;
        Ok(var_25.pop())
    }
}

pub(crate) fn deser_header_stop_failback_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_tag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_26 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_26.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_26.len()
        )))
    } else {
        let mut var_26 = var_26;
        Ok(var_26.pop())
    }
}

pub(crate) fn deser_header_tag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_terminate_recovery_instances_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_27 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_27.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_27.len()
        )))
    } else {
        let mut var_27 = var_27;
        Ok(var_27.pop())
    }
}

pub(crate) fn deser_header_terminate_recovery_instances_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_untag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_28 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_28.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_28.len()
        )))
    } else {
        let mut var_28 = var_28;
        Ok(var_28.pop())
    }
}

pub(crate) fn deser_header_untag_resource_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_update_failback_replication_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_29 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_29.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_29.len()
        )))
    } else {
        let mut var_29 = var_29;
        Ok(var_29.pop())
    }
}

pub(crate) fn deser_header_update_failback_replication_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_update_launch_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_30 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_30.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_30.len()
        )))
    } else {
        let mut var_30 = var_30;
        Ok(var_30.pop())
    }
}

pub(crate) fn deser_header_update_launch_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_update_replication_configuration_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_31 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_31.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_31.len()
        )))
    } else {
        let mut var_31 = var_31;
        Ok(var_31.pop())
    }
}

pub(crate) fn deser_header_update_replication_configuration_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn deser_header_update_replication_configuration_template_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i64>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_32 = aws_smithy_http::header::read_many_primitive::<i64>(headers)?;
    if var_32.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_32.len()
        )))
    } else {
        let mut var_32 = var_32;
        Ok(var_32.pop())
    }
}

pub(crate) fn deser_header_update_replication_configuration_template_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("Retry-After").iter();
    aws_smithy_http::header::one_or_none(headers)
}
