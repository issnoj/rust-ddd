use std::panic;
use std::rc::Rc;
use chrono::{Duration, Local};
use crate::application::use_case::borrow_book_use_case::BorrowBookUseCase;
use crate::application::use_case::change_plan_use_case::ChangePlanUseCase;
use crate::application::use_case::return_book_use_case::ReturnBookUseCase;
use crate::domain::value_object::plan::Plan;
use crate::infrastructure::query::get_borrowing_book_list_query::GetBorrowingBookListQuery;
use crate::infrastructure::repository::book_circulation_repository::BookCirculationRepository;
use crate::infrastructure::repository::book_repository::BookRepository;
use crate::infrastructure::repository::user_repository::UserRepository;

mod domain;
mod infrastructure;
mod application;

fn main() {
    // 事前準備
    let user_repository = Rc::new(UserRepository::new());
    let book_repository = Rc::new(BookRepository::new());
    let book_circulation_repository = Rc::new(BookCirculationRepository::new());
    let user_id = user_repository.create("Alice");
    let book_id_1 = book_repository.create("AAA");
    let book_id_2 = book_repository.create("BBB");
    let book_id_3 = book_repository.create("CCC");
    let user = user_repository.get_by_id(user_id.as_str()).unwrap();
    let borrow_book_use_case = BorrowBookUseCase::new(
        book_repository.clone(),
        book_circulation_repository.clone(),
    );
    let change_plan_use_case = ChangePlanUseCase::new(
        user_repository.clone(),
        book_circulation_repository.clone(),
    );
    let return_book_use_case = ReturnBookUseCase::new(
        book_repository.clone(),
        book_circulation_repository.clone(),
    );
    let get_borrowing_book_list_query = GetBorrowingBookListQuery::new(
        user_repository.clone(),
        book_repository.clone(),
        book_circulation_repository.clone(),
    );


    // 1. 本を 2 冊借りる
    borrow_book_use_case.execute(user.clone(), book_id_1.clone());
    borrow_book_use_case.execute(user.clone(), book_id_2);

    // 2. 3 冊目を借りようとするとエラーになる
    assert!(panic::catch_unwind(|| {
        borrow_book_use_case.execute(user.clone(), book_id_3.clone());
    }).is_err());

    // 3. フリープランからスタンダードプランに変更する
    change_plan_use_case.execute(user.clone(), Plan::Standard);

    // 4. 3 冊目を借りる
    let user = user_repository.get_by_id(user_id.as_str()).unwrap();
    borrow_book_use_case.execute(user.clone(), book_id_3);

    // 5. スタンダードプランからフリープランに変更しようとするとエラーになる
    assert!(panic::catch_unwind(|| {
        change_plan_use_case.execute(user.clone(), Plan::Free);
    }).is_err());

    // 6. 借りた日付より前の日付で返そうとするとエラーになる
    assert!(panic::catch_unwind(|| {
        let date = Local::now() - Duration::days(1);
        return_book_use_case.execute(user.clone(), book_id_1.clone(), date);
    }).is_err());

    // 7. 借りた本を返す
    let date = Local::now() + Duration::days(1);
    return_book_use_case.execute(user.clone(), book_id_1, date);

    // 8. スタンダードプランからフリープランに変更する
    change_plan_use_case.execute(user.clone(), Plan::Free);

    // 9. 借りている本を確認する
    println!("借りている本：");
    for v in get_borrowing_book_list_query.execute(user_id.as_str()) {
        println!(
            "\t{},{},{}",
            v.get("id").unwrap(),
            v.get("title").unwrap(),
            v.get("borrowDate").unwrap()
        );
    }
}