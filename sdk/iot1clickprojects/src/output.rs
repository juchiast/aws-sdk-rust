// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateProjectOutput {}
/// See [`UpdateProjectOutput`](crate::output::UpdateProjectOutput).
pub mod update_project_output {

    /// A builder for [`UpdateProjectOutput`](crate::output::UpdateProjectOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateProjectOutput`](crate::output::UpdateProjectOutput).
        pub fn build(self) -> crate::output::UpdateProjectOutput {
            crate::output::UpdateProjectOutput {}
        }
    }
}
impl UpdateProjectOutput {
    /// Creates a new builder-style object to manufacture [`UpdateProjectOutput`](crate::output::UpdateProjectOutput).
    pub fn builder() -> crate::output::update_project_output::Builder {
        crate::output::update_project_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdatePlacementOutput {}
/// See [`UpdatePlacementOutput`](crate::output::UpdatePlacementOutput).
pub mod update_placement_output {

    /// A builder for [`UpdatePlacementOutput`](crate::output::UpdatePlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdatePlacementOutput`](crate::output::UpdatePlacementOutput).
        pub fn build(self) -> crate::output::UpdatePlacementOutput {
            crate::output::UpdatePlacementOutput {}
        }
    }
}
impl UpdatePlacementOutput {
    /// Creates a new builder-style object to manufacture [`UpdatePlacementOutput`](crate::output::UpdatePlacementOutput).
    pub fn builder() -> crate::output::update_placement_output::Builder {
        crate::output::update_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput {}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput {}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>The tags (metadata key/value pairs) which you have assigned to the resource.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>The tags (metadata key/value pairs) which you have assigned to the resource.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags (metadata key/value pairs) which you have assigned to the resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>The tags (metadata key/value pairs) which you have assigned to the resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListProjectsOutput {
    /// <p>An object containing the list of projects.</p>
    #[doc(hidden)]
    pub projects: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListProjectsOutput {
    /// <p>An object containing the list of projects.</p>
    pub fn projects(&self) -> std::option::Option<&[crate::model::ProjectSummary]> {
        self.projects.as_deref()
    }
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListProjectsOutput`](crate::output::ListProjectsOutput).
pub mod list_projects_output {

    /// A builder for [`ListProjectsOutput`](crate::output::ListProjectsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) projects: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `projects`.
        ///
        /// To override the contents of this collection use [`set_projects`](Self::set_projects).
        ///
        /// <p>An object containing the list of projects.</p>
        pub fn projects(mut self, input: crate::model::ProjectSummary) -> Self {
            let mut v = self.projects.unwrap_or_default();
            v.push(input);
            self.projects = Some(v);
            self
        }
        /// <p>An object containing the list of projects.</p>
        pub fn set_projects(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ProjectSummary>>,
        ) -> Self {
            self.projects = input;
            self
        }
        /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListProjectsOutput`](crate::output::ListProjectsOutput).
        pub fn build(self) -> crate::output::ListProjectsOutput {
            crate::output::ListProjectsOutput {
                projects: self.projects,
                next_token: self.next_token,
            }
        }
    }
}
impl ListProjectsOutput {
    /// Creates a new builder-style object to manufacture [`ListProjectsOutput`](crate::output::ListProjectsOutput).
    pub fn builder() -> crate::output::list_projects_output::Builder {
        crate::output::list_projects_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListPlacementsOutput {
    /// <p>An object listing the requested placements.</p>
    #[doc(hidden)]
    pub placements: std::option::Option<std::vec::Vec<crate::model::PlacementSummary>>,
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListPlacementsOutput {
    /// <p>An object listing the requested placements.</p>
    pub fn placements(&self) -> std::option::Option<&[crate::model::PlacementSummary]> {
        self.placements.as_deref()
    }
    /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListPlacementsOutput`](crate::output::ListPlacementsOutput).
pub mod list_placements_output {

    /// A builder for [`ListPlacementsOutput`](crate::output::ListPlacementsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) placements: std::option::Option<std::vec::Vec<crate::model::PlacementSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `placements`.
        ///
        /// To override the contents of this collection use [`set_placements`](Self::set_placements).
        ///
        /// <p>An object listing the requested placements.</p>
        pub fn placements(mut self, input: crate::model::PlacementSummary) -> Self {
            let mut v = self.placements.unwrap_or_default();
            v.push(input);
            self.placements = Some(v);
            self
        }
        /// <p>An object listing the requested placements.</p>
        pub fn set_placements(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::PlacementSummary>>,
        ) -> Self {
            self.placements = input;
            self
        }
        /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token used to retrieve the next set of results - will be effectively empty if there are no further results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListPlacementsOutput`](crate::output::ListPlacementsOutput).
        pub fn build(self) -> crate::output::ListPlacementsOutput {
            crate::output::ListPlacementsOutput {
                placements: self.placements,
                next_token: self.next_token,
            }
        }
    }
}
impl ListPlacementsOutput {
    /// Creates a new builder-style object to manufacture [`ListPlacementsOutput`](crate::output::ListPlacementsOutput).
    pub fn builder() -> crate::output::list_placements_output::Builder {
        crate::output::list_placements_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetDevicesInPlacementOutput {
    /// <p>An object containing the devices (zero or more) within the placement.</p>
    #[doc(hidden)]
    pub devices:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl GetDevicesInPlacementOutput {
    /// <p>An object containing the devices (zero or more) within the placement.</p>
    pub fn devices(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.devices.as_ref()
    }
}
/// See [`GetDevicesInPlacementOutput`](crate::output::GetDevicesInPlacementOutput).
pub mod get_devices_in_placement_output {

    /// A builder for [`GetDevicesInPlacementOutput`](crate::output::GetDevicesInPlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) devices: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `devices`.
        ///
        /// To override the contents of this collection use [`set_devices`](Self::set_devices).
        ///
        /// <p>An object containing the devices (zero or more) within the placement.</p>
        pub fn devices(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.devices.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.devices = Some(hash_map);
            self
        }
        /// <p>An object containing the devices (zero or more) within the placement.</p>
        pub fn set_devices(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.devices = input;
            self
        }
        /// Consumes the builder and constructs a [`GetDevicesInPlacementOutput`](crate::output::GetDevicesInPlacementOutput).
        pub fn build(self) -> crate::output::GetDevicesInPlacementOutput {
            crate::output::GetDevicesInPlacementOutput {
                devices: self.devices,
            }
        }
    }
}
impl GetDevicesInPlacementOutput {
    /// Creates a new builder-style object to manufacture [`GetDevicesInPlacementOutput`](crate::output::GetDevicesInPlacementOutput).
    pub fn builder() -> crate::output::get_devices_in_placement_output::Builder {
        crate::output::get_devices_in_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DisassociateDeviceFromPlacementOutput {}
/// See [`DisassociateDeviceFromPlacementOutput`](crate::output::DisassociateDeviceFromPlacementOutput).
pub mod disassociate_device_from_placement_output {

    /// A builder for [`DisassociateDeviceFromPlacementOutput`](crate::output::DisassociateDeviceFromPlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DisassociateDeviceFromPlacementOutput`](crate::output::DisassociateDeviceFromPlacementOutput).
        pub fn build(self) -> crate::output::DisassociateDeviceFromPlacementOutput {
            crate::output::DisassociateDeviceFromPlacementOutput {}
        }
    }
}
impl DisassociateDeviceFromPlacementOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateDeviceFromPlacementOutput`](crate::output::DisassociateDeviceFromPlacementOutput).
    pub fn builder() -> crate::output::disassociate_device_from_placement_output::Builder {
        crate::output::disassociate_device_from_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeProjectOutput {
    /// <p>An object describing the project.</p>
    #[doc(hidden)]
    pub project: std::option::Option<crate::model::ProjectDescription>,
}
impl DescribeProjectOutput {
    /// <p>An object describing the project.</p>
    pub fn project(&self) -> std::option::Option<&crate::model::ProjectDescription> {
        self.project.as_ref()
    }
}
/// See [`DescribeProjectOutput`](crate::output::DescribeProjectOutput).
pub mod describe_project_output {

    /// A builder for [`DescribeProjectOutput`](crate::output::DescribeProjectOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) project: std::option::Option<crate::model::ProjectDescription>,
    }
    impl Builder {
        /// <p>An object describing the project.</p>
        pub fn project(mut self, input: crate::model::ProjectDescription) -> Self {
            self.project = Some(input);
            self
        }
        /// <p>An object describing the project.</p>
        pub fn set_project(
            mut self,
            input: std::option::Option<crate::model::ProjectDescription>,
        ) -> Self {
            self.project = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeProjectOutput`](crate::output::DescribeProjectOutput).
        pub fn build(self) -> crate::output::DescribeProjectOutput {
            crate::output::DescribeProjectOutput {
                project: self.project,
            }
        }
    }
}
impl DescribeProjectOutput {
    /// Creates a new builder-style object to manufacture [`DescribeProjectOutput`](crate::output::DescribeProjectOutput).
    pub fn builder() -> crate::output::describe_project_output::Builder {
        crate::output::describe_project_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribePlacementOutput {
    /// <p>An object describing the placement.</p>
    #[doc(hidden)]
    pub placement: std::option::Option<crate::model::PlacementDescription>,
}
impl DescribePlacementOutput {
    /// <p>An object describing the placement.</p>
    pub fn placement(&self) -> std::option::Option<&crate::model::PlacementDescription> {
        self.placement.as_ref()
    }
}
/// See [`DescribePlacementOutput`](crate::output::DescribePlacementOutput).
pub mod describe_placement_output {

    /// A builder for [`DescribePlacementOutput`](crate::output::DescribePlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) placement: std::option::Option<crate::model::PlacementDescription>,
    }
    impl Builder {
        /// <p>An object describing the placement.</p>
        pub fn placement(mut self, input: crate::model::PlacementDescription) -> Self {
            self.placement = Some(input);
            self
        }
        /// <p>An object describing the placement.</p>
        pub fn set_placement(
            mut self,
            input: std::option::Option<crate::model::PlacementDescription>,
        ) -> Self {
            self.placement = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribePlacementOutput`](crate::output::DescribePlacementOutput).
        pub fn build(self) -> crate::output::DescribePlacementOutput {
            crate::output::DescribePlacementOutput {
                placement: self.placement,
            }
        }
    }
}
impl DescribePlacementOutput {
    /// Creates a new builder-style object to manufacture [`DescribePlacementOutput`](crate::output::DescribePlacementOutput).
    pub fn builder() -> crate::output::describe_placement_output::Builder {
        crate::output::describe_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteProjectOutput {}
/// See [`DeleteProjectOutput`](crate::output::DeleteProjectOutput).
pub mod delete_project_output {

    /// A builder for [`DeleteProjectOutput`](crate::output::DeleteProjectOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteProjectOutput`](crate::output::DeleteProjectOutput).
        pub fn build(self) -> crate::output::DeleteProjectOutput {
            crate::output::DeleteProjectOutput {}
        }
    }
}
impl DeleteProjectOutput {
    /// Creates a new builder-style object to manufacture [`DeleteProjectOutput`](crate::output::DeleteProjectOutput).
    pub fn builder() -> crate::output::delete_project_output::Builder {
        crate::output::delete_project_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeletePlacementOutput {}
/// See [`DeletePlacementOutput`](crate::output::DeletePlacementOutput).
pub mod delete_placement_output {

    /// A builder for [`DeletePlacementOutput`](crate::output::DeletePlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeletePlacementOutput`](crate::output::DeletePlacementOutput).
        pub fn build(self) -> crate::output::DeletePlacementOutput {
            crate::output::DeletePlacementOutput {}
        }
    }
}
impl DeletePlacementOutput {
    /// Creates a new builder-style object to manufacture [`DeletePlacementOutput`](crate::output::DeletePlacementOutput).
    pub fn builder() -> crate::output::delete_placement_output::Builder {
        crate::output::delete_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateProjectOutput {}
/// See [`CreateProjectOutput`](crate::output::CreateProjectOutput).
pub mod create_project_output {

    /// A builder for [`CreateProjectOutput`](crate::output::CreateProjectOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CreateProjectOutput`](crate::output::CreateProjectOutput).
        pub fn build(self) -> crate::output::CreateProjectOutput {
            crate::output::CreateProjectOutput {}
        }
    }
}
impl CreateProjectOutput {
    /// Creates a new builder-style object to manufacture [`CreateProjectOutput`](crate::output::CreateProjectOutput).
    pub fn builder() -> crate::output::create_project_output::Builder {
        crate::output::create_project_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreatePlacementOutput {}
/// See [`CreatePlacementOutput`](crate::output::CreatePlacementOutput).
pub mod create_placement_output {

    /// A builder for [`CreatePlacementOutput`](crate::output::CreatePlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CreatePlacementOutput`](crate::output::CreatePlacementOutput).
        pub fn build(self) -> crate::output::CreatePlacementOutput {
            crate::output::CreatePlacementOutput {}
        }
    }
}
impl CreatePlacementOutput {
    /// Creates a new builder-style object to manufacture [`CreatePlacementOutput`](crate::output::CreatePlacementOutput).
    pub fn builder() -> crate::output::create_placement_output::Builder {
        crate::output::create_placement_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AssociateDeviceWithPlacementOutput {}
/// See [`AssociateDeviceWithPlacementOutput`](crate::output::AssociateDeviceWithPlacementOutput).
pub mod associate_device_with_placement_output {

    /// A builder for [`AssociateDeviceWithPlacementOutput`](crate::output::AssociateDeviceWithPlacementOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`AssociateDeviceWithPlacementOutput`](crate::output::AssociateDeviceWithPlacementOutput).
        pub fn build(self) -> crate::output::AssociateDeviceWithPlacementOutput {
            crate::output::AssociateDeviceWithPlacementOutput {}
        }
    }
}
impl AssociateDeviceWithPlacementOutput {
    /// Creates a new builder-style object to manufacture [`AssociateDeviceWithPlacementOutput`](crate::output::AssociateDeviceWithPlacementOutput).
    pub fn builder() -> crate::output::associate_device_with_placement_output::Builder {
        crate::output::associate_device_with_placement_output::Builder::default()
    }
}
