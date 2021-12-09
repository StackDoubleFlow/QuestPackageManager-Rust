mod dependency_internal;
pub type Dependency = dependency_internal::Dependency;
pub type AdditionalDependencyData = dependency_internal::AdditionalDependencyData;

mod shared_dependency;
pub type SharedDependency = shared_dependency::SharedDependency;
