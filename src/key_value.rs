use std::collections::HashMap;

pub fn new_list() -> HashMap<&str, &[u8]>{
    let list = HashMap::new();

    return list;
}

pub fn insert_value(mut list_name: HashMap<&str, &[u8]>, key: &str, value: &[u8]){
    list_name.insert(key, value);
}

pub fn get_value(mut list_name: HashMap<&str, &[u8]>, key: &str) -> &[u8]{
    let value = list_name.get(key);
    return String::from(value).as_bytes();
}