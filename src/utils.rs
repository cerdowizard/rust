use rand::Rng;
#[derive(Debug)]
#[allow(dead_code)]
pub struct ApiResponse {
    code: u32,
    message: String,
}

pub fn gen_id() -> String{
    let uui = rand::thread_rng().gen_range(0..=100000);
    let new_id = uui.to_string();
    new_id
}


// let new_user = UserInfo::create_user_info()
            //     .map(|user| {
            //         println!("User created: {:?}", user);
            //         let new_account =
            //             BankInfo::create_account_info(user.id.clone(), user.name.clone());
            //         // Use new_user.id and new_user.name as needed
            //     })
            //     .unwrap_or_else(|api_response| {
            //         println!("Error creating user: {:?}", api_response);
            //         // Handle error if necessary
            //     });
            // Assuming create_user_info returns a Result<UserInfo, ()>
            // match UserInfo::create_user_info() {
            //     Ok(new_user) => {
            //         // Handle the new user as needed
            //         println!("User created: {:?}", new_user);
            //     }
            //     Err(_) => {
            //         println!("Failed to create user.");
            //     }
            // }