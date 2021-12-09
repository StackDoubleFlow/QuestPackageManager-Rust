use std::process;

use pubgrub::{
    error::PubGrubError,
    report::{DefaultStringReporter, Reporter},
};

use self::provider::DependencyProvider;
use crate::data::{
    package::{PackageConfig, SharedPackageConfig},
    qpackages,
};

mod provider;
mod semver;

pub fn resolve(root: &PackageConfig) -> impl Iterator<Item = SharedPackageConfig> + '_ {
    let provider = DependencyProvider::new(root);
    match pubgrub::solver::resolve(&provider, root.info.id.clone(), root.info.version.clone()) {
        Ok(deps) => deps
            .into_iter()
            .filter(move |(id, version)| !(id == &root.info.id && version == &root.info.version))
            .map(|(id, version)| qpackages::get_shared_package(&id, &version.into())),
        Err(PubGrubError::NoSolution(tree)) => {
            let report = DefaultStringReporter::report(&tree);
            eprintln!("failed to resolve dependencies: \n{}", report);
            process::exit(1)
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    }
}
