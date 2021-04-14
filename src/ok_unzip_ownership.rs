fn get_str_column(table: &Vec<(String, usize)>) -> Vec<&str> {
    let (column_1, _): (Vec<String>, Vec<usize>) = table.iter().unzip();
    let mut return_value = Vec::new();
    for s in column_1.iter() {
        return_value.push(s.as_str());
    }
    return_value
}

pub fn main() {
    let table = vec![("a".to_string(), 1)];
    let str_column = get_str_column(&table);
}
