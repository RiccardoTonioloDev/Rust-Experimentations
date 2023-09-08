use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new(); //Esplicitiamo il tipo della
    //HashMap, usante le stringhe

    loop {
        println!("Commands available:");
        println!("1) Add <EmployeeName> to <Department>");
        println!("2) List <Department>");
        println!("3) List All");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); //Lettura dell'input

        let input = input.trim(); //Pulitura da spazi all'inizio o alla fine
        let command_words: Vec<&str> = input.split_whitespace().collect(); //Divisione dagli spazi

        match command_words[0] { //In base alla parola trovata come prima parola, si decide come
            //proseguire.
            "Add" if command_words.len() == 4 && command_words[2] == "to" => {
                let employee_name = command_words[1].to_string();
                let department_name = command_words[3].to_string();
                add_employee(&mut company, &employee_name, &department_name);
            }
            "List" if command_words.len() == 2 => {
                match command_words[1] {
                    "All" => {
                        list_all(&company);
                    }
                    department_name => {
                        list_department(&company, department_name);
                    }
                }
            }
            _ => println!("Invalid command, try again."),
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, employee_name: &str, department_name: &str) {
    company
        .entry(department_name.to_string())
        .or_insert(Vec::new())
        .push(employee_name.to_string());
    println!("Added {} to {}", employee_name, department_name);
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort();

    for department_name in departments {
        list_department(&company, department_name);
    }
}

fn list_department(company: &HashMap<String, Vec<String>>, department_name: &str) {
    if let Some(employees) = company.get(department_name) { //Se non esiste siamo nel caso del None
        //e entriamo quindi nella condizione di "else".
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();

        println!("Employees in {} (sorted alphabetically):", department_name);
        for employee in sorted_employees {
            println!("  - {}", employee);
        }
    } else {
        println!("Department {} does not exist.", department_name);
    }
}

