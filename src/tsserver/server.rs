use crate::find_argument;

fn find_argument_string_array(arg_name: &str) -> Vec<String> {
    let arg = find_argument(arg_name);
    if arg.is_none() {
        return vec![];
    }
    let arg = arg.unwrap();
    arg.split(",")
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub fn start() {
    unimplemented!()
}
