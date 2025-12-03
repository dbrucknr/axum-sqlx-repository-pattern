use super::AuthenticationModule;

pub trait AuthenticationController {
    fn login() -> impl Future<Output = String>;
    fn authenticate() -> impl Future<Output = String>;
    fn logout() -> impl Future<Output = String>;
}

impl AuthenticationController for AuthenticationModule {
    async fn login() -> String {
        String::from("Login successful")
    }

    async fn authenticate() -> String {
        String::from("Authentication successful")
    }

    async fn logout() -> String {
        String::from("Logout successful")
    }
}
