use std::borrow::Borrow;

use pubgrub::{range::Range, solver::Dependencies};
use pubgrub::solver::DependencyProvider as PubGrubDependencyProvider;

use super::semver::{req_to_range, Version};
use crate::data::{package::PackageConfig, qpackages};

pub struct  DependencyProvider<'a> {
    root: &'a PackageConfig,
}

impl<'a> DependencyProvider<'a> {
    pub fn new(root: &'a PackageConfig) -> Self {
        Self { root }
    }
}

impl DependencyProvider<'_> {
    fn get_package_versions(&self, id: &str) -> Vec<qpackages::PackageVersion> {
        let mut result = qpackages::get_versions(id.borrow());
        // we add ourselves to the gotten versions, so the local version always can be resolved as most ideal
        if id == self.root.info.id {
            result.push(qpackages::PackageVersion {
                id: self.root.info.id.clone(),
                version: self.root.info.version.clone(),
            });
        }

        result
    }
}

impl PubGrubDependencyProvider<String, Version> for DependencyProvider<'_> {
    fn choose_package_version<T: Borrow<String>, U: Borrow<Range<Version>>>(
        &self,
        potential_packages: impl Iterator<Item = (T, U)>,
    ) -> Result<(T, Option<Version>), Box<dyn std::error::Error>> {
        Ok(pubgrub::solver::choose_package_with_fewest_versions(
            |id| {
                self.get_package_versions(id.borrow())
                    .into_iter()
                    .map(|pv: qpackages::PackageVersion| pv.version.into())
            },
            potential_packages,
        ))
    }

    fn get_dependencies(
        &self,
        id: &String,
        version: &Version,
    ) -> Result<Dependencies<String, Version>, Box<dyn std::error::Error>> {
        if id == &self.root.info.id && version == &self.root.info.version {
            let deps = self
                .root
                .dependencies
                .clone()
                .into_iter()
                .map(|dep| {
                    let id = dep.id;
                    let version = req_to_range(dep.version_range);
                    (id, version)
                })
                .collect();
            Ok(Dependencies::Known(deps))
        } else {
            let mut package = qpackages::get_shared_package(id, &version.clone().into());
            // remove any private dependencies
            package
                .config
                .dependencies
                .retain(|dep| !dep.additional_data.is_private.unwrap_or(false));

            let deps = package
                .config
                .dependencies
                .into_iter()
                .map(|dep| {
                    let id = dep.id;
                    let version = Range::any();
                    (id, version)
                })
                .collect();
            Ok(Dependencies::Known(deps))
        }
    }
}
