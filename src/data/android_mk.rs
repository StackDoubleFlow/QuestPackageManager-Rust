use std::io::{Read, Write};

use crate::data::{
    dependency::SharedDependency,
    package::{PackageConfig, SharedPackageConfig},
};

#[derive(Debug, Default)]
pub struct Module {
    pub prefix_lines: Vec<String>,
    pub id: String,
    pub src: Vec<String>,
    pub export_includes: String,
    pub static_libs: Vec<String>,
    pub shared_libs: Vec<String>,
    pub ld_libs: Vec<String>,
    pub c_flags: Vec<String>,
    pub export_c_flags: Vec<String>,
    pub cpp_flags: Vec<String>,
    pub c_includes: Vec<String>,
    pub cpp_features: Vec<String>,
    pub extra_lines: Vec<String>,
    pub build_line: String,
}

impl ToString for Module {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for pre in self.prefix_lines.iter() {
            result.push_str(&format!("{}\n", pre));
        }

        result.push_str(&format!("LOCAL_MODULE := {}\n", self.id));
        if !self.export_includes.is_empty() {
            result.push_str(&format!(
                "LOCAL_EXPORT_C_INCLUDES := {}\n",
                self.export_includes
            ));
        }
        if !self.export_c_flags.is_empty() {
            result.push_str("LOCAL_EXPORT_C_FLAGS := ");
            for feat in self.export_c_flags.iter() {
                result.push_str(&format!(" {}", feat));
            }
            result.push('\n');
        }
        if self.src.len() == 1 {
            result.push_str(&format!(
                "LOCAL_SRC_FILES := {}\n",
                self.src.get(0).unwrap()
            ));
        } else {
            for src in self.src.iter() {
                result.push_str(&format!("LOCAL_SRC_FILES += {}\n", src));
            }
        }
        if !self.shared_libs.is_empty() {
            for lib in self.shared_libs.iter() {
                result.push_str(&format!("LOCAL_SHARED_LIBRARIES += {}\n", lib));
            }
        }
        if !self.static_libs.is_empty() {
            for lib in self.static_libs.iter() {
                result.push_str(&format!("LOCAL_STATIC_LIBRARIES += {}\n", lib));
            }
        }
        if !self.ld_libs.is_empty() {
            result.push_str("LOCAL_LDLIBS += ");
            for ld in self.ld_libs.iter() {
                result.push_str(&format!(" {}", ld));
            }
            result.push('\n');

            //result.push_str(&format!("LOCAL_LDLIBS += {}\n", string.Join(' ', self.LdLibs));
        }
        if !self.c_flags.is_empty() {
            result.push_str("LOCAL_CFLAGS += ");
            for flag in self.c_flags.iter() {
                result.push_str(&format!(" {}", flag));
            }
            result.push('\n');

            //result.push_str(&format!("LOCAL_CFLAGS += {}\n", string.Join(' ', self.CFlags));
        }
        if !self.cpp_flags.is_empty() {
            result.push_str("LOCAL_CPPFLAGS += ");
            for flag in self.cpp_flags.iter() {
                result.push_str(&format!(" {}", flag));
            }
            result.push('\n');

            //result.push_str(&format!("LOCAL_CPPFLAGS += {}\n", string.Join(' ', self.CppFlags));
        }
        if !self.c_includes.is_empty() {
            result.push_str("LOCAL_C_INCLUDES += ");
            for incl in self.c_includes.iter() {
                result.push_str(&format!(" {}", incl));
            }
            result.push('\n');

            //result.push_str(&format!("LOCAL_C_INCLUDES += {}\n", string.Join(' ', self.CIncludes));
        }
        if !self.cpp_features.is_empty() {
            result.push_str("LOCAL_CPP_FEATURES += ");
            for feat in self.cpp_features.iter() {
                result.push_str(&format!(" {}", feat));
            }
            result.push('\n');

            //result.push_str(&format!("LOCAL_CPP_FEATURES += {}\n", string.Join(' ', self.CppFeatures));
        }

        // Suffix all unknown lines, hopefully this is good enough
        for e in self.extra_lines.iter() {
            result.push_str(e);
        }
        result.push_str(&format!("{}\n", self.build_line));

        result
    }
}

#[derive(Debug, Default)]
pub struct AndroidMk {
    pub prefix: Vec<String>,
    pub modules: Vec<Module>,
    pub suffix: Vec<String>,
}

#[derive(Eq, PartialEq, Debug)]
enum Concat {
    None,
    Set,
    Add,
}

fn break_string(line: &str, concat_type: &mut Concat) -> Option<String> {
    let ind = line.find('=');
    // if found
    if let Some(index) = ind {
        match &line.chars().nth(index - 1).unwrap() {
            '+' => {
                *concat_type = Concat::Add;
            }
            ':' => {
                *concat_type = Concat::Set;
            }
            _ => {
                *concat_type = Concat::None;
            }
        }
        return Some(line[index + 1..].trim_start().to_string());
    }

    *concat_type = Concat::None;
    None
}

fn parse_line(line: &str) -> Vec<String> {
    let mut lst = Vec::<String>::new();
    let mut temp = String::new();
    let mut wildcard = false;
    let mut escaped_parenth = false;
    let mut escaped_single = false;
    let mut escaped_double = false;
    let mut escape_next = false;
    for c in line.chars() {
        if escape_next {
            escape_next = false;
            temp.push(c);
            continue;
        }
        if wildcard && c == '(' {
            escaped_parenth = true;
        }
        wildcard = false;

        match c {
            '$' => wildcard = true,
            '\\' => escape_next = true,
            '\'' => escaped_single = !escaped_single,
            '\"' => escaped_double = !escaped_double,
            ')' => escaped_parenth = false,
            ' ' => {
                if !escaped_single && !escaped_double && !escaped_parenth {
                    lst.push(temp);
                    temp = String::new();
                    continue;
                }
            }
            _ => {}
        }

        temp.push(c);
    }
    // Always add at least one
    lst.push(temp);
    lst
}

impl AndroidMk {
    pub fn read() -> Option<AndroidMk> {
        if let Ok(mut file) = std::fs::File::open("Android.mk") {
            let mut android_mk_string = String::new();
            file.read_to_string(&mut android_mk_string)
                .expect("Reading data failed");
            Some(Self::from_str(&android_mk_string))
        } else {
            None
        }
    }

    pub fn write(&self) {
        let android_mk_string = self.to_string();
        let mut file = std::fs::File::create("Android.mk").expect("Opening Android.mk failed");
        file.write_all(android_mk_string.as_bytes())
            .expect("write failed");
    }

    fn from_str(s: &str) -> AndroidMk {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut in_module = false;
        let mut first_module_found = false;
        let mut mk = AndroidMk {
            ..Default::default()
        };
        let mut module = Module {
            ..Default::default()
        };

        for line in lines.iter() {
            if !first_module_found {
                mk.prefix.push(line.to_string());
            } else if !in_module {
                module.prefix_lines.push(line.to_string());
            } else {
                // Check if mod end
                if line.starts_with("include $(")
                    || line.starts_with("rwildcard=$")
                    || line.starts_with("LOCAL_PATH")
                    || line.starts_with("TARGET_ARCH_ABI")
                {
                    module.build_line = line.to_string();
                    mk.modules.push(module);
                    // Exit module with build statement
                    in_module = false;
                    // Create new module to populate prefix for
                    module = Module {
                        ..Default::default()
                    };
                    continue;
                }
                // Parse line
                let mut concat_type = Concat::None;
                let parsed_opt = break_string(line, &mut concat_type);
                if parsed_opt.is_none() {
                    // If line can't be parsed, skip
                    continue;
                }

                let parsed = parsed_opt.unwrap();

                if line.starts_with("LOCAL_MODULE") {
                    module.id = parsed;
                } else if line.starts_with("LOCAL_SRC_FILES") {
                    if concat_type == Concat::Set {
                        module.src.clear();
                    }

                    module.src.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_EXPORT_C_INCLUDES") {
                    if concat_type == Concat::Set {
                        module.export_includes.clear();
                    }
                    module.export_includes.push_str(&parsed);
                } else if line.starts_with("LOCAL_EXPORT_CFLAGS") {
                    if concat_type == Concat::Set {
                        module.export_c_flags.clear();
                    }
                    module.export_c_flags.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_SHARED_LIBRARIES") {
                    if concat_type == Concat::Set {
                        module.shared_libs.clear();
                    }
                    module.shared_libs.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_STATIC_LIBRARIES") {
                    if concat_type == Concat::Set {
                        module.static_libs.clear();
                    }
                    module.static_libs.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_LDLIBS") {
                    if concat_type == Concat::Set {
                        module.ld_libs.clear();
                    }
                    module.ld_libs.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_CFLAGS") {
                    if concat_type == Concat::Set {
                        module.c_flags.clear();
                    }
                    module.c_flags.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_CPPFLAGS") {
                    if concat_type == Concat::Set {
                        module.cpp_flags.clear();
                    }
                    module.cpp_flags.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_C_INCLUDES") {
                    if concat_type == Concat::Set {
                        module.c_includes.clear();
                    }
                    module.c_includes.append(&mut parse_line(&parsed));
                } else if line.starts_with("LOCAL_CPP_FEATURES") {
                    if concat_type == Concat::Set {
                        module.cpp_features.clear();
                    }
                    module.cpp_features.append(&mut parse_line(&parsed));
                } else {
                    module.extra_lines.push(line.to_string());
                }
            }

            if line.starts_with("include $(CLEAR_VARS)") {
                let mut size = mk.prefix.len();
                if !first_module_found && size > 0 {
                    let mut index = size - 2;
                    let mut prefix_line = mk.prefix.get(index).expect("vector size was < 2");
                    if prefix_line.starts_with('#') {
                        module.prefix_lines.push(prefix_line.clone());
                        mk.prefix.remove(index);
                    }

                    size = mk.prefix.len();
                    index = size - 1;
                    prefix_line = mk.prefix.get(index).expect("vector size was < 1");

                    if prefix_line.starts_with("include $(CLEAR_VARS)") {
                        module.prefix_lines.push(prefix_line.clone());
                        mk.prefix.remove(index);
                    }
                }

                in_module = true;
                first_module_found = true;
            }
        }

        // Add last portion of module prefix to suffix of mk
        mk.suffix.append(&mut module.prefix_lines);
        mk
    }

    pub fn update_shared_dependency(&mut self, shared_dependency: &SharedDependency) {
        let shared_package = shared_dependency.get_shared_package();
        let package = PackageConfig::read();

        let static_linking = shared_package
            .config
            .info
            .additional_data
            .static_linking
            .unwrap_or(false);

        let override_so_name = shared_package.config.info.additional_data.override_so_name;
        let id = if let Some(override_so_name) = &override_so_name {
            override_so_name[3..override_so_name.len() - if static_linking { 2 } else { 3 }]
                .to_string()
        } else {
            shared_dependency.dependency.id.clone()
        };

        let export_includes = format!(
            " {}/{}",
            package.dependencies_dir.display(),
            shared_dependency.dependency.id.clone()
        );

        // existing module
        let src = if let Some(override_so_name) = &override_so_name {
            format!(
                "{}/{}",
                package.dependencies_dir.display(),
                override_so_name
            )
        } else {
            format!(
                "{}/{}_{}",
                package.dependencies_dir.display(),
                id,
                shared_dependency.version.to_string().replace('.', "_")
            )
        };

        if let Some(module) = self
            .modules
            .iter_mut()
            .find(|submodule| submodule.id.contains(&id))
        {
            module.export_includes.clear();
            module.export_includes = export_includes;

            module.src.clear();
            module.src.push(src);
        } else {
            // module did not exist, add it / make it!
            let mut module = Module {
                id: format!(
                    "{}_{}",
                    id.clone(),
                    shared_dependency.version.to_string().replace('.', "_")
                ),
                export_includes,
                ..Default::default()
            };

            module.src.push(src);

            module.prefix_lines.push(format!(
                "# Creating prebuilt for dependency: {} - version: {}",
                shared_dependency.dependency.id.clone(),
                shared_dependency.version.to_string()
            ));

            module
                .prefix_lines
                .push("include $(CLEAR_VARS)".to_string());

            module.extra_lines.push(if static_linking {
                "include $(PREBUILT_STATIC_LIBRARY)".to_string()
            } else {
                "include $(PREBUILT_SHARED_LIBRARY)".to_string()
            });

            self.modules.insert(0, module);
        }
    }

    pub fn update_shared_package(&mut self, shared_package: &SharedPackageConfig) {
        for shared_dependency in shared_package.restored_dependencies.iter() {
            self.update_shared_dependency(shared_dependency);
        }

        let static_linking = shared_package
            .config
            .info
            .additional_data
            .static_linking
            .unwrap_or(false);

        let override_so_name = &shared_package.config.info.additional_data.override_so_name;
        let id = if let Some(override_so_name) = &override_so_name {
            override_so_name[3..override_so_name.len() - if static_linking { 2 } else { 3 }]
                .to_string()
        } else {
            shared_package.config.info.id.clone()
        };

        self.modules.last_mut().unwrap().id = if let Some(override_so_name) = override_so_name {
            override_so_name[3..override_so_name.len() - if static_linking { 2 } else { 3 }]
                .to_string()
        } else {
            id
        };

        let mut shared_libs = Vec::new();
        let mut static_libs = Vec::new();

        for submodule in self.modules.iter() {
            if submodule
                .extra_lines
                .iter()
                .any(|l| l.contains("PREBUILT_SHARED_LIBRARY"))
            {
                // it's shared
                shared_libs.push(submodule.id.clone());
            } else if submodule
                .extra_lines
                .iter()
                .any(|l| l.contains("PREBUILT_STATIC_LIBRARY"))
            {
                static_libs.push(submodule.id.clone());
            }
        }

        self.modules.last_mut().unwrap().shared_libs = shared_libs;
        self.modules.last_mut().unwrap().static_libs = static_libs;
    }
}

impl ToString for AndroidMk {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for line in self.prefix.iter() {
            result.push_str(&format!("{}\n", line));
        }

        for module in self.modules.iter() {
            result.push_str(&format!("{}\n", module.to_string()));
        }

        for line in self.suffix.iter() {
            result.push_str(&format!("{}\n", line));
        }

        result
    }
}
