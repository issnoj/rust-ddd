use ulid::Ulid;
use crate::domain::entity::user_plan::UserPlan;
use crate::domain::value_object::plan::Plan;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Ulid,
    pub name: String,
    pub user_plan: UserPlan,
}

impl User {
    fn new(id: Ulid, name: &str, user_plan: UserPlan) -> Self {
        Self {
            id,
            name: name.to_string(),
            user_plan,
        }
    }

    pub fn create(name: &str) -> Self {
        let user_plan = UserPlan::create(Plan::Free);
        Self::new(Ulid::new(), name, user_plan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        // given
        let name = "name";

        // when
        let user = User::create(name);

        // then
        assert_eq!(user.id.to_string().len(), 26);
        assert_eq!(user.name, name);
        assert_eq!(user.user_plan.plan, Plan::Free);
    }
}