// use std::{
//     pin::Pin,
//     sync::{Arc, Mutex},
// };

// use crate::domain::{
//     result::UserDomainResult,
//     user::{EmailStatus, User},
// };

// use super::db_transactions::{DBTransaction, MockTransaction};
// use super::user_repository_trait::UserRepositoryTrait;

// //Type aliases for each method's closure signature
// type CreateAccountFn = Arc<Mutex<dyn FnMut(User) -> UserDomainResult<()> + Send>>;
// type UpsertUserFn =
//     Arc<Mutex<dyn FnMut(User, Option<&mut DBTransaction>) -> UserDomainResult<()> + Send>>;
// type MakeModeratorFn = UpdateClosure;
// type ChangeUsernameFn = UpdateClosure;
// type AwardBageFn = UpdateClosure;
// type RevokeBadgeFn = UpdateClosure;
// type BanUserFn = UpdateClosure;
// type UnbanUserFn = UpdateClosure;
// type GetUserByIdFn = Arc<Mutex<dyn FnMut(&str) -> UserDomainResult<Option<User>> + Send>>;
// type GetUserByUsernameOrEmailFn =
//     Arc<Mutex<dyn FnMut(&str, &str) -> UserDomainResult<Option<User>> + Send>>;
// type UserExistsFn =
//     Arc<Mutex<dyn FnMut(&str, &str, Option<EmailStatus>) -> UserDomainResult<bool> + Send>>;
// type RunInTransactionFn = Arc<
//     Mutex<
//         dyn FnMut(
//                 Box<dyn FnOnce(Option<DBTransaction>) -> BoxFutureUserDomainResult + Send>,
//             ) -> UserDomainResult<()>
//             + Send,
//     >,
// >;

// // Helper type for boxed update function
// type BoxedUpdateFn = Box<dyn FnOnce(&mut User) + Send>;
// type UpdateClosure = Arc<Mutex<dyn FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send>>;
// type BoxFutureUserDomainResult = Pin<Box<dyn Future<Output = UserDomainResult<()>> + Send>>;

// pub struct MockUserRepositoryTrait {
//     pub create_account_fn: Option<CreateAccountFn>,
//     pub make_moderator_fn: Option<MakeModeratorFn>,
//     pub change_username_fn: Option<ChangeUsernameFn>,
//     pub award_badge_fn: Option<AwardBageFn>,
//     pub revoke_badge_fn: Option<RevokeBadgeFn>,
//     pub ban_user_fn: Option<BanUserFn>,
//     pub unban_user_fn: Option<UnbanUserFn>,
//     pub get_user_by_id_fn: Option<GetUserByIdFn>,
//     pub get_user_by_username_or_email_fn: Option<GetUserByUsernameOrEmailFn>,
//     pub user_exists_fn: Option<UserExistsFn>,
//     pub upsert_user_fn: Option<UpsertUserFn>,
//     pub run_in_transaction_fn: Option<RunInTransactionFn>,
// }

// impl MockUserRepositoryTrait {
//     pub fn new() -> Self {
//         Self {
//             create_account_fn: None,
//             make_moderator_fn: None,
//             change_username_fn: None,
//             award_badge_fn: None,
//             revoke_badge_fn: None,
//             ban_user_fn: None,
//             unban_user_fn: None,
//             get_user_by_id_fn: None,
//             get_user_by_username_or_email_fn: None,
//             user_exists_fn: None,
//             upsert_user_fn: None,
//             run_in_transaction_fn: None,
//         }
//     }
//     // Setters for expectations
//     pub fn expect_create_account<F>(&mut self, f: F)
//     where
//         F: FnMut(User) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.create_account_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_make_moderator<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.make_moderator_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_change_username<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.change_username_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_award_badge<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.award_badge_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_revoke_badge<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.revoke_badge_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_ban_user<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.ban_user_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_unban_user<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, BoxedUpdateFn) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.unban_user_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_get_user_by_id<F>(&mut self, f: F)
//     where
//         F: FnMut(&str) -> UserDomainResult<Option<User>> + Send + 'static,
//     {
//         self.get_user_by_id_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_get_user_by_email_or_username<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, &str) -> UserDomainResult<Option<User>> + Send + 'static,
//     {
//         self.get_user_by_username_or_email_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_user_exists<F>(&mut self, f: F)
//     where
//         F: FnMut(&str, &str, Option<EmailStatus>) -> UserDomainResult<bool> + Send + 'static,
//     {
//         self.user_exists_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_upsert_user<F>(&mut self, f: F)
//     where
//         F: FnMut(User, Option<&mut DBTransaction>) -> UserDomainResult<()> + Send + 'static,
//     {
//         self.upsert_user_fn = Some(Arc::new(Mutex::new(f)));
//     }
//     pub fn expect_run_in_transaction<F>(&mut self, f: F)
//     where
//         F: FnMut(
//                 Box<dyn FnOnce(Option<DBTransaction>) -> BoxFutureUserDomainResult + Send>,
//             ) -> UserDomainResult<()>
//             + Send
//             + 'static,
//     {
//         self.run_in_transaction_fn = Some(Arc::new(Mutex::new(f)));
//     }
// }

// // #[async_trait::async_trait]
// // impl UserRepositoryTrait for MockUserRepositoryTrait {
// //     async fn create_account(&self, user: User) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .create_account_fn
// //             .as_ref()
// //             .expect("create_account called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user)
// //     }
// //     async fn make_moderator<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .make_moderator_fn
// //             .as_ref()
// //             .expect("make_moderator called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }

// //     async fn change_username<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .change_username_fn
// //             .as_ref()
// //             .expect("change_username called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }
// //     async fn award_badge<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .award_badge_fn
// //             .as_ref()
// //             .expect("award_badge called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }

// //     async fn revoke_badge<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .revoke_badge_fn
// //             .as_ref()
// //             .expect("revoke_badge called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }
// //     async fn ban_user<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .ban_user_fn
// //             .as_ref()
// //             .expect("ban_user called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }
// //     async fn unban_user<F: FnOnce(&mut User) + Send + 'static>(
// //         &self,
// //         user_id: &str,
// //         update_fn: F,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .unban_user_fn
// //             .as_ref()
// //             .expect("unban_user called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id, Box::new(update_fn))
// //     }
// //     async fn get_user_by_id(&self, user_id: &str) -> UserDomainResult<Option<User>> {
// //         let mut_ref = self
// //             .get_user_by_id_fn
// //             .as_ref()
// //             .expect("get_user_by_id called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user_id)
// //     }
// //     async fn get_user_by_username_or_email(
// //         &self,
// //         email: &str,
// //         username: &str,
// //     ) -> UserDomainResult<Option<User>> {
// //         let mut_ref = self
// //             .get_user_by_username_or_email_fn
// //             .as_ref()
// //             .expect("get_user_by_username_or_email called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(email, username)
// //     }
// //     async fn user_exists(
// //         &self,
// //         username: &str,
// //         email: &str,
// //         email_status: Option<EmailStatus>,
// //     ) -> UserDomainResult<bool> {
// //         let mut_ref = self
// //             .user_exists_fn
// //             .as_ref()
// //             .expect("user_exists called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(username, email, email_status)
// //     }

// //     async fn upsert_user<'a>(
// //         &self,
// //         user: User,
// //         session: Option<&'a mut DBTransaction>,
// //     ) -> UserDomainResult<()> {
// //         let mut_ref = self
// //             .upsert_user_fn
// //             .as_ref()
// //             .expect("upsert_user called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();
// //         (closure)(user, session)
// //     }
// //     async fn run_in_transaction<F, Fut>(&self, f: F) -> UserDomainResult<()>
// //     where
// //         F: FnOnce(Option<DBTransaction>) -> Fut + Send + 'static,
// //         Fut: Future<Output = UserDomainResult<()>> + Send + 'static,
// //     {
// //         let mut_ref = self
// //             .run_in_transaction_fn
// //             .as_ref()
// //             .expect("run_in_transaction called but no expectation set");
// //         let mut closure = mut_ref.lock().unwrap();

// //         let boxed_f = Box::new(move |trx| Box::pin(f(trx)) as BoxFutureUserDomainResult);
// //         (closure)(boxed_f)
// //     }
// // }
