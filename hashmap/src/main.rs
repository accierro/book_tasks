use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company. For example, “Add
// Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
// a list of all people in a department or all people in the company by
// department, sorted alphabetically.

fn main() {
    let mut employee_by_department = HashMap::new();
    employee_by_department.insert(String::from("Sales"), Vec::<String>::new());
    employee_by_department.insert(String::from("IT"), Vec::<String>::new());
    employee_by_department.insert(String::from("Engineering"), Vec::<String>::new());

    add_employee(&mut employee_by_department, "Add Vlad to IT");
    add_employee(&mut employee_by_department, "Add John to IT");
    add_employee(&mut employee_by_department, "Add Willow to Sales");

    println!("{:?}", employee_by_department)
}

fn add_employee(hmap: &mut HashMap<String, Vec<String>>, command: &str) {
    let re = Regex::new(r"Add (\w*) to (\w*)").unwrap();
    for cap in re.captures_iter(command) {
        match hmap.entry((&cap[2]).to_string()) {
            Entry::Occupied(mut e) => e.get_mut().push((&cap[1]).to_string()),
            Entry::Vacant(_e) => println!("There is no {} department", &cap[2]),
        }
    }
}
