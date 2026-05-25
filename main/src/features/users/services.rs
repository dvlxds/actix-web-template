pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        println!(
            "UserService 实例化 - 线程: {:?}",
            std::thread::current().id()
        );
        Self {}
    }

    pub fn get_profile(&self, user_id: u32) -> String {
        format!("User profile for {}", user_id)
    }
}
