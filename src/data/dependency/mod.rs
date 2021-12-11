mod dependency_internal;
/// A dependency is a library that you want to use for your mod
pub type Dependency = dependency_internal::Dependency;
/// Additional dependency data can be used to configure a dependency a certain way to make it work the way you want it to
///
/// Really it's the same as AdditionalPackageData though
pub type AdditionalDependencyData = crate::data::package::AdditionalPackageData;

mod shared_dependency;
/// A shared dependency is a dependency that, when someone wants to use your lib as a dependency, is used for more dependency resolution.
///
/// It is also used to check out how it was configured when you generated the library
pub type SharedDependency = shared_dependency::SharedDependency;
