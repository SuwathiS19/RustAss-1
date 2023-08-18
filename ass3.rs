
use std::io;

struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    fn new() -> Self {
        Employee {
            employee_name: String::new(),
            employee_id: 0,
            email: String::new(),
            age: 0,
            phone_number: String::new(),
        }
    }
}

fn get_employee_details() -> Employee {
    let mut employee = Employee::new();
    
    println!("Enter Employee Name:");
    io::stdin().read_line(&mut employee.employee_name).expect("Failed to read line");
    
    println!("Enter Employee ID:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    employee.employee_id = input.trim().parse().expect("Invalid input");
    
    println!("Enter Employee Email:");
    input.clear();
    io::stdin().read_line(&mut employee.email).expect("Failed to read line");
    
    println!("Enter Employee Age:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    employee.age = input.trim().parse().expect("Invalid input");
    
    println!("Enter Employee Phone Number:");
    input.clear();
    io::stdin().read_line(&mut employee.phone_number).expect("Failed to read line");
    
    employee
}

fn find_employee_by_id(employees: &[Employee], target_id: i32) -> Option<&Employee> {
    employees.iter().find(|emp| emp.employee_id == target_id)
}

fn find_employees_by_age(employees: &[Employee], target_age: i32) -> Vec<&Employee> {
    employees.iter().filter(|emp| emp.age == target_age).collect()
}

fn main() {
    println!("Enter the number of employees:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_employees: usize = input.trim().parse().expect("Invalid input");
    
    let mut employee_list: Vec<Employee> = Vec::new();
    
    for _ in 0..num_employees {
        let employee = get_employee_details();
        employee_list.push(employee);
    }
    
    println!("Enter an Employee ID to search:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target_id: i32 = input.trim().parse().expect("Invalid input");
    
    if let Some(employee) = find_employee_by_id(&employee_list, target_id) {
        println!("Employee found:");
        println!("Employee Name: {}", employee.employee_name);
        println!("Employee ID: {}", employee.employee_id);
        println!("Employee Email: {}", employee.email);
        println!("Employee Age: {}", employee.age);
        println!("Employee Phone Number: {}", employee.phone_number);
    } else {
        println!("Employee not found.");
    }
    
    println!("Enter an Age to search employees:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target_age: i32 = input.trim().parse().expect("Invalid input");
    
    let employees_with_age = find_employees_by_age(&employee_list, target_age);
    if !employees_with_age.is_empty() {
        println!("Employees with age {}:", target_age);
        for employee in employees_with_age {
            println!("Employee Name: {}", employee.employee_name);
            println!("Employee ID: {}", employee.employee_id);
            println!("Employee Email: {}", employee.email);
            println!("Employee Age: {}", employee.age);
            println!("Employee Phone Number: {}", employee.phone_number);
            println!();
        }
    } else {
        println!("No employees found with age {}.", target_age);
    }
}
