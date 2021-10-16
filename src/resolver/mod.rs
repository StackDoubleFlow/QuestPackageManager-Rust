use std::process;

use ::semver::Version;
use pubgrub::{
    error::PubGrubError,
    report::{DefaultStringReporter, Reporter},
};

use self::provider::DependencyProvider;
use crate::data::package::PackageConfig;

mod provider;
mod semver;

pub fn resolve(root: &PackageConfig) -> impl Iterator<Item = (String, Version)> + '_ {
    let provider = DependencyProvider::new(root);
    match pubgrub::solver::resolve(&provider, root.info.id.clone(), root.info.version.clone()) {
        Ok(deps) => deps
            .into_iter()
            .map(|(id, version)| (id, version.into()))
            .filter(move |(id, version)| !(id == &root.info.id && version == &root.info.version)),
        Err(PubGrubError::NoSolution(tree)) => {
            let report = DefaultStringReporter::report(&tree);
            eprintln!("{}", report);
            process::exit(1)
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    }
}
