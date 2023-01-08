use crate::domain::value_object::plan::Plan;

#[derive(Debug, Clone)]
pub struct UserPlan {
    pub plan: Plan,
}

impl UserPlan {
    fn new(plan: Plan) -> Self {
        Self {
            plan
        }
    }

    pub fn create(plan: Plan) -> Self {
        Self::new(plan)
    }

    pub fn change_plan(&mut self, plan: Plan) {
        self.plan = plan;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        // given
        let plan = Plan::Free;

        // when
        let user_plan = UserPlan::create(plan.clone());

        // then
        assert_eq!(user_plan.plan, plan);
    }
}