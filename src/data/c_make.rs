use std::{collections::HashMap, str::FromStr, string::ParseError};

// example of a cmake file for ndk https://github.com/Smertig/beat-singer-quest/blob/master/CMakeLists.txt

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CMakeList {
    pub minimum_version: String,
    pub project: String,
    pub sets: HashMap<String, String>,
    pub compile_options: Vec<String>,
    pub compile_definitions: Vec<String>,
    pub subdirectories: Vec<String>,
    pub include_directories: Vec<String>,
    pub link_directories: Vec<String>,
    pub shared_library: Vec<SharedLibrary>,
    pub target_include_directories: Vec<SharedLibrary>,
    pub target_link_libraries: Vec<SharedLibrary>,
    pub extra_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedLibrary {
    pub id: String,
    pub protection: String,
    pub files: Vec<String>,
}

impl CMakeList {}

impl std::str::FromStr for SharedLibrary {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('\n', " ");
        let s = s.replace('\t', " ");
        let s = s.replace('\r', "");

        let split: Vec<&str> = s.split(' ').collect();

        let mut files: Vec<String> = split[2..]
            .iter()
            .map(|f| f.to_string().trim().to_string())
            .collect();

        files.retain(|f| !f.is_empty());
        Ok(SharedLibrary {
            id: split.get(0).unwrap().to_string(),
            protection: split.get(1).unwrap().to_string(),
            files,
        })
    }
}

fn sanitize(s: &str) -> String {
    let mut sanitized = String::new();

    for line in s.split('\n') {
        if !line.starts_with('#') {
            sanitized.push_str(&format!("{}\n", line));
        }
    }

    sanitized
}

impl FromStr for CMakeList {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut minimum_version = String::new();
        let mut project = String::new();
        let mut sets = HashMap::<String, String>::new();
        let mut compile_options = Vec::<String>::new();
        let mut compile_definitions = Vec::<String>::new();
        let mut subdirectories = Vec::<String>::new();
        let mut include_directories = Vec::<String>::new();
        let mut link_directories = Vec::<String>::new();
        let mut shared_library = Vec::<SharedLibrary>::new();
        let mut target_include_directories = Vec::<SharedLibrary>::new();
        let mut target_link_libraries = Vec::<SharedLibrary>::new();
        let mut extra_lines = Vec::<String>::new();

        let sanitized = sanitize(s);
        // every "line" in a cmake file is ended with a )
        for line in sanitized.split(')') {
            let line = line.trim();
            let content;
            if let Some(idx) = line.find('(') {
                content = line[idx + 1..].to_string()
            } else {
                continue;
            }

            if line.starts_with("cmake_minimum_required") {
                minimum_version = content;
            } else if line.starts_with("project") {
                project = content;
            } else if line.starts_with("set") {
                let pair: Vec<&str> = content.split(' ').collect();
                sets.insert(
                    pair.first().unwrap().to_string(),
                    pair.last().unwrap().to_string(),
                );
            } else if line.starts_with("add_compile_options") {
                for option in content
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|op| op[1..].to_string())
                {
                    compile_options.push(option);
                }
            } else if line.starts_with("add_compile_definitions") {
                for definition in content.split(' ') {
                    compile_definitions.push(definition.to_string());
                }
            } else if line.starts_with("add_subdirectory") {
                subdirectories.push(content);
            } else if line.starts_with("include_directories") {
                include_directories.push(content);
            } else if line.starts_with("link_directories") {
                link_directories.push(content);
            } else if line.starts_with("add_library") {
                shared_library.push(SharedLibrary::from_str(&content).unwrap());
            } else if line.starts_with("target_include_directories") {
                target_include_directories.push(SharedLibrary::from_str(&content).unwrap());
            } else if line.starts_with("target_link_libraries") {
                target_link_libraries.push(SharedLibrary::from_str(&content).unwrap());
            } else {
                extra_lines.push(format!("{}\n)", line));
            }
        }

        Ok(CMakeList {
            minimum_version,
            project,
            sets,
            compile_options,
            compile_definitions,
            subdirectories,
            include_directories,
            link_directories,
            shared_library,
            target_include_directories,
            target_link_libraries,
            extra_lines,
        })
    }
}

impl ToString for CMakeList {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!(
            "cmake_minimum_required({})\n",
            self.minimum_version
        ));
        result.push_str(&format!("project({})\n", self.project));
        result.push('\n');

        for (key, value) in self.sets.iter() {
            result.push_str(&format!("set({} {})\n", key, value));
        }

        result.push('\n');
        result.push_str("add_compile_options(");
        let mut first = true;
        for option in self.compile_options.iter() {
            if first {
                result.push_str(&format!("-{}", option));
                first = false;
            } else {
                result.push_str(&format!(" -{}", option));
            }
        }
        result.push_str(")\n");
        for definition in self.compile_definitions.iter() {
            result.push_str(&format!("add_compile_definitions({})\n", definition));
        }
        result.push('\n');

        for subdirectory in self.subdirectories.iter() {
            result.push_str(&format!("add_subdirectory({})\n", subdirectory));
        }
        result.push('\n');

        for include_directory in self.include_directories.iter() {
            result.push_str(&format!("include_directories({})\n", include_directory));
        }
        result.push('\n');

        for link_directory in self.link_directories.iter() {
            result.push_str(&format!("link_directories({})\n", link_directory));
        }
        result.push('\n');

        for lib in self.shared_library.iter() {
            if lib.files.len() == 1 {
                result.push_str(&format!(
                    "add_library({} {} {})\n",
                    lib.id,
                    lib.protection,
                    lib.files.first().unwrap()
                ));
            } else {
                result.push_str(&format!("add_library({} {}\n", lib.id, lib.protection));
                for file in lib.files.iter() {
                    result.push_str(&format!("\t{}\n", file));
                }
                result.push_str(")\n");
            }
        }
        result.push('\n');

        for lib in self.target_include_directories.iter() {
            if lib.files.len() == 1 {
                result.push_str(&format!(
                    "target_include_directories({} {} {})\n",
                    lib.id,
                    lib.protection,
                    lib.files.first().unwrap()
                ));
            } else {
                result.push_str(&format!(
                    "target_include_directories({} {}\n",
                    lib.id, lib.protection
                ));
                for file in lib.files.iter() {
                    result.push_str(&format!("\t{}\n", file));
                }
                result.push_str(")\n");
            }
        }
        result.push('\n');

        for lib in self.target_link_libraries.iter() {
            if lib.files.len() == 1 {
                result.push_str(&format!(
                    "target_link_libraries({} {} {})\n",
                    lib.id,
                    lib.protection,
                    lib.files.first().unwrap()
                ));
            } else {
                result.push_str(&format!(
                    "target_link_libraries({} {}\n",
                    lib.id, lib.protection
                ));
                for file in lib.files.iter() {
                    result.push_str(&format!("\t{}\n", file));
                }
                result.push_str(")\n");
            }
        }

        if !self.extra_lines.is_empty() {
            result.push('\n');
            result.push_str("# Misc extra lines: \n");
            for extra in self.extra_lines.iter() {
                result.push_str(&format!("{}\n", extra));
            }
        }

        result
    }
}
