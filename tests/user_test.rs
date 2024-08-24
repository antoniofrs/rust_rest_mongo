use app::config::logging::init_logging;
use app::dto::user_dto::InsertUserDto;
use app::model::crud::user::User;
use app::repository::user_repository::{UserRepository, UserRepositoryTrait};
use app::service::user_service::{UserService, UserServiceTrait};
use common::models::test_insert_user_sto;
use mongodb::bson::oid::ObjectId;
use crate::common::test_containers::get_test_database;

mod common;



#[tokio::test]
async fn user_is_created_and_found() {
    init_logging();
    let database = get_test_database().await;
    let user_repository = UserRepository::init(database);
    let user_service = UserService::init(user_repository);

    let insert_user_dto = test_insert_user_sto();

    let user_dto = user_service.save(insert_user_dto).await.unwrap();
    let generated_id = ObjectId::parse_str(&user_dto.id).unwrap();

    let by_id = user_service.find_by_id(generated_id).await.unwrap();

    assert_eq!(user_dto, by_id);
}

#[tokio::test]
async fn user_is_updated_and_found() {
    let database = get_test_database().await;
    let user_repository = UserRepository::init(database);
    let user_service = UserService::init(user_repository);

    let insert_user_dto = test_insert_user_sto();

    let user_dto = user_service.save(insert_user_dto).await.unwrap();
    let generated_id = ObjectId::parse_str(&user_dto.id).unwrap();

    let updated_user_dto = InsertUserDto {
        name: "new_name".into(),
        surname: "new_surname".into(),
        email: "new_email@email.com".into(),
    };

    user_service.update(generated_id, updated_user_dto).await.unwrap();

    let updated_user = user_service.find_by_id(generated_id).await.unwrap();

    assert_eq!(updated_user.name, "new_name");
    assert_eq!(updated_user.surname, "new_surname");
    assert_eq!(updated_user.email, "new_email@email.com");
}

#[tokio::test]
async fn user_is_deleted_and_not_found() {
    let database = get_test_database().await;
    let user_repository = UserRepository::init(database);
    let user_service = UserService::init(user_repository);

    let insert_user_dto = test_insert_user_sto();

    let user_dto = user_service.save(insert_user_dto).await.unwrap();
    let generated_id = ObjectId::parse_str(&user_dto.id).unwrap();

    let user = User {
        id: generated_id.clone(),
        name: user_dto.name.clone(),
        surname: user_dto.surname.clone(),
        email: user_dto.email.clone(),
    };

    user_service.delete(user.id).await.unwrap();

    let deleted_user = user_service.find_by_id(generated_id).await;
    assert!(deleted_user.is_err());
}


#[tokio::test]
async fn user_is_created_and_found_by_id() {
    let database = get_test_database().await;
    let user_repository = UserRepository::init(database);
    let user_service = UserService::init(user_repository);

    let insert_user_dto = test_insert_user_sto();

    let user_dto = user_service.save(insert_user_dto).await.unwrap();
    let generated_id = ObjectId::parse_str(&user_dto.id).unwrap();

    let found_user = user_service.find_by_id(generated_id).await.unwrap();

    assert_eq!(found_user.id, user_dto.id);
    assert_eq!(found_user.name, user_dto.name);
    assert_eq!(found_user.surname, user_dto.surname);
    assert_eq!(found_user.email, user_dto.email);
}

