mod mall;

use mall::*;

pub fn biggest_store(mall: Mall) -> Option<floor::store::Store> {
    let mut biggest_store: Option<floor::store::Store> = None;
    let mut max_size = 0;

    for floor in mall.floors {
        for store in floor.stores {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                biggest_store = Some(store.clone());
            }
        }
    }

    biggest_store
}

pub fn highest_paid_employee(mall: Mall) -> Vec<floor::store::employee::Employee> {
    let mut highest_paid_employees: Vec<floor::store::employee::Employee> = Vec::new();
    let mut max_salary = 0.0;

    for floor in mall.floors {
        for store in floor.stores {
            for employee in store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    highest_paid_employees.clear();
                    highest_paid_employees.push(employee.clone());
                } else if employee.salary == max_salary {
                    highest_paid_employees.push(employee.clone());
                }
            }
        }
    }

    highest_paid_employees
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut total_employees = 0;

    for floor in mall.floors {
        for store in floor.stores {
            total_employees += store.employees.len();
        }
    }

    total_employees
}

pub fn check_for_securities(mall: &mut Mall, new_guards: Vec<mall::guard::Guard>) {
    let total_floor_size: u64 = mall.floors.iter().map(|floor| {
        floor.stores.iter().map(|store| store.square_meters).sum::<u64>()
    }).sum();

    let required_guards = total_floor_size / 200;
    let current_guards_count = mall.guards.len();

    if current_guards_count < required_guards as usize {
        for guard in new_guards {
            mall.hire_guard(guard);
        }
    }
}


pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let working_hours = employee.working_hours;
                let total_hours = working_hours.1 - working_hours.0;
                let increase = if total_hours > 10 { 0.1 } else { -0.1 };
                if increase > 0.0 {
                    employee.raise(employee.salary * increase);
                } else {
                    employee.cut(employee.salary * (-increase));
                }
            }
        }
    }
}
