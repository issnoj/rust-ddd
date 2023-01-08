use std::panic::RefUnwindSafe;
use std::rc::Rc;
use crate::domain::entity::user::User;
use crate::domain::value_object::plan::Plan;
use crate::infrastructure::repository::book_circulation_repository::BookCirculationRepository;
use crate::infrastructure::repository::user_repository::UserRepository;

pub struct ChangePlanUseCase {
    user_repository: Rc<UserRepository>,
    book_circulation_repository: Rc<BookCirculationRepository>,
}

impl ChangePlanUseCase {
    pub fn new(
        user_repository: Rc<UserRepository>,
        book_circulation_repository: Rc<BookCirculationRepository>,
    ) -> Self {
        Self {
            user_repository,
            book_circulation_repository,
        }
    }

    pub fn execute(&self, mut user: User, plan: Plan) {
        if user.user_plan.plan == plan {
            panic!("同じプランです");
        }

        let max_borrowing_count = plan.max_borrowing_count();

        let now_borrowing_count = self.book_circulation_repository.borrowing_count(user.id);

        if now_borrowing_count > max_borrowing_count {
            panic!("現在{}冊貸出中のため、{}にプランを変更できません", now_borrowing_count, plan.name());
        }

        user.user_plan.change_plan(plan.clone());

        self.user_repository.update(user.clone());

        println!("プランを{}に変更しました", plan.name());
    }
}

impl RefUnwindSafe for ChangePlanUseCase {}