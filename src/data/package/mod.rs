mod compile_options;
pub type CompileOptions = compile_options::CompileOptions;

mod package_config;
pub type PackageConfig = package_config::PackageConfig;
pub type PackageInfo = package_config::PackageInfo;
pub type AdditionalPackageData = package_config::AdditionalPackageData;

mod shared_package_config;
pub type SharedPackageConfig = shared_package_config::SharedPackageConfig;
