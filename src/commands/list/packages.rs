use owo_colors::OwoColorize;

pub fn execute_packages_list() {
    let ids = crate::data::qpackages::get_packages();
    if !ids.is_empty() {
        println!(
            "Found {} packages on qpackages.com",
            ids.len().bright_yellow()
        );
        let mut idx = 0;
        for id in ids.iter() {
            println!("{}", id);
            idx += 1;
            if idx % 5 == 0 {
                println!();
                idx = 0;
            }
        }
    } else {
        println!("qpackages.com returned 0 packages, is something wrong?");
    }
}
