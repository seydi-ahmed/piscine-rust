pub mod mall;
pub use mall::floor::store;
pub use mall::floor::*;
pub use mall::*;

pub fn biggest_store(m: Mall) -> floor::store::Store {
    let mut list: Vec<store::Store> = Vec::new();

    for f in m.floors {
        for s in f.stores {
            list.push(s);
        }
    }

    list.sort_unstable_by(|a, b| a.square_meters.cmp(&b.square_meters));
    let biggest = &list[list.len() - 1];

    biggest.clone()
}

pub fn highest_paid_employee(m: Mall) -> Vec<store::employee::Employee> {
    let mut list: Vec<store::employee::Employee> = Vec::new();

    for f in m.floors {
        for s in f.stores {
            for e in s.employees {
                list.push(e);
            }
        }
    }

    list.sort_unstable_by(|a, b| a.salary.partial_cmp(&b.salary).unwrap());

    let highest = &list[list.len() - 1].salary;

    list = list
        .iter()
        .map(|e| e.clone())
        .filter(|s| s.salary == *highest)
        .collect::<Vec<_>>();

    list
}

pub fn nbr_of_employees(m: Mall) -> usize {
    let mut num_employees = m.guards.len();

    for f in m.floors {
        for s in f.stores {
            num_employees += s.employees.len();
        }
    }

    num_employees
}

pub fn check_for_securities(m: &mut Mall, guards: Vec<guard::Guard>) {
    let mut num_areas = 0_u64;

    for f in &m.floors {
        num_areas += f.size_limit / 200_u64;
    }

    let guards_to_add = m.guards.len().abs_diff(num_areas as usize);

    for i in 0..guards_to_add + 1 {
        m.hire_guard(guards[i].clone());
    }
}

pub fn cut_or_raise(m: &mut Mall) {
    for f in &mut m.floors {
        for s in &mut f.stores {
            for e in &mut s.employees {
                if e.working_hours.0.abs_diff(e.working_hours.1) > 10 {
                    let rate = 0.1 * &e.salary;
                    e.salary += rate;
                } else {
                    let rate = 0.1 * &e.salary;
                    e.salary -= rate;
                }
            }
        }
    }
}
