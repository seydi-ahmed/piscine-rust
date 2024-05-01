
pub mod mall {
    pub mod floor {
        pub mod store {
            pub mod employee {
                pub struct Employee {
                    pub name: String,
                    pub age: u8,
                    pub working_hours: (u8, u8),
                    pub salary: f64,
                }
            }

            pub struct Store {
                pub name: String,
                pub square_meters: u64,
                pub employees: Vec<employee::Employee>,
            }
        }
    }

    pub struct Mall {
        pub name: String,
        pub guards: Vec<guard::Guard>,
        pub floors: Vec<floor::Floor>,
    }
}

pub use crate::mall::floor::store::employee::Employee;
pub use crate::mall::floor::store::Store;
pub use crate::mall::Mall;



pub use std::collections::HashMap;
pub use crate::mall::*;

pub fn biggest_store(mall: Mall) -> Store {
    let mut biggest = mall.floors[0].stores[0].clone();
    for floor in &mall.floors {
        for store in &floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store.clone();
            }
        }
    }
    biggest
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_paid: Vec<Employee> = Vec::new();
    let mut max_salary = 0.0;

    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push(employee.clone());
                } else if employee.salary == max_salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }
    highest_paid
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut count = 0;
    for floor in &mall.floors {
        for store in &floor.stores {
            count += store.employees.len();
        }
    }
    count
}

pub fn check_for_securities<Guard>(mall: &mut crate::mall::Mall, additional_guards: Vec<crate::mall::guard::Guard>,) {
    let mut total_floor_size = 0;
    for floor in &mall.floors {
        total_floor_size += floor.size_limit;
    }
    let mut total_guards = 0;
    for guard in &mall.guards {
        total_guards += guard.years_experience as usize;
    }
    let required_guards = total_floor_size / 200;
    if total_guards < required_guards {
        mall.guards.extend(additional_guards);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let (entry_hour, exit_hour) = employee.working_hours;
                let worked_hours = (exit_hour - entry_hour) as usize;
                if worked_hours > 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}
