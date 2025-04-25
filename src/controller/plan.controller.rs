use crate::models::plan_model::Plan;
use std::sync::Mutex;
use uuid::Uuid;
use lazy_static::lazy_static;

lazy_static! {
    static ref PLAN_STORE: Mutex<Vec<Plan>> = Mutex::new(Vec::new());
}

pub fn create_plan(name: String, description: String) -> Plan {
    let plan = Plan {
        id: Uuid::new_v4(),
        name,
        description,
    };
    PLAN_STORE.lock().unwrap().push(plan.clone());
    plan
}

pub fn get_plan_by_id(id: Uuid) -> Option<Plan> {
    PLAN_STORE.lock().unwrap().iter().cloned().find(|p| p.id == id)
}

pub fn get_all_plans() -> Vec<Plan> {
    PLAN_STORE.lock().unwrap().clone()
}

pub fn update_plan(id: Uuid, name: String, description: String) -> Option<Plan> {
    let mut store = PLAN_STORE.lock().unwrap();
    if let Some(plan) = store.iter_mut().find(|p| p.id == id) {
        plan.name = name;
        plan.description = description;
        return Some(plan.clone());
    }
    None
}

pub fn delete_plan(id: Uuid) -> bool {
    let mut store = PLAN_STORE.lock().unwrap();
    let len_before = store.len();
    store.retain(|p| p.id != id);
    len_before != store.len()
}
