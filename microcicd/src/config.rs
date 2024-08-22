use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;

pub static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let config_content = fs::read_to_string("./config/setting")
        .expect("Unable to read config file");

    pro_to_map(config_content)
});

pub static TASK: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let config_content = fs::read_to_string("./config/task")
        .expect("Unable to read config file");

    pro_to_map(config_content)
});


fn pro_to_map(pro: String) -> HashMap<String, String> {
    let map = pro.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, "=");
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect::<HashMap<String, String>>();

    map
}