use std::io::{Read, Write};

pub struct Module {
    pub prefix_lines: Vec<String>,
    pub id: Option<String>,
    pub src: Vec<String>,
    pub export_includes: Option<String>,
    pub static_libs: Vec<String>,
    pub shared_libs: Vec<String>,
    pub ld_libs: Vec<String>,
    pub c_flags: Vec<String>,
    pub export_c_flags: Vec<String>,
    pub cpp_flags: Vec<String>,
    pub c_includes: Vec<String>,
    pub cpp_features: Vec<String>,
    pub extra_lines: Vec<String>,
    pub build_line: Option<String>
}

impl Default for Module {
    fn default() -> Module {
        Module {
            prefix_lines: Vec::default(),
            id: Option::default(),
            src: Vec::default(),
            export_includes: Option::default(),
            static_libs: Vec::default(),
            shared_libs: Vec::default(),
            ld_libs: Vec::default(),
            c_flags: Vec::default(),
            export_c_flags: Vec::default(),
            cpp_flags: Vec::default(),
            c_includes: Vec::default(),
            cpp_features: Vec::default(),
            extra_lines: Vec::default(),
            build_line: Option::default(),
        }
    }
}

pub struct AndroidMk {
    pub prefix: Vec<String>,
    pub modules: Vec<Module>,
    pub suffix: Vec<String>
}

impl Default for AndroidMk {
    fn default() -> AndroidMk {
        AndroidMk {
            prefix: Vec::default(),
            modules: Vec::default(),
            suffix: Vec::default()
        }
    }
}

#[allow(dead_code)]
impl AndroidMk {
    pub fn read() -> AndroidMk {
        let mut file = std::fs::File::open("Android.mk").expect("Opening Android.mk failed");
        let mut android_mk_string = String::new();
        file.read_to_string(&mut android_mk_string).expect("Reading data failed");

        AndroidMk {..Default::default()}
    }

    pub fn write()
    {
        let android_mk_string = "TODO";
        let mut file = std::fs::File::open("Android.mk").expect("Opening Android.mk failed");

        file.write_all(android_mk_string.as_bytes()).expect("write failed");
    }
}

