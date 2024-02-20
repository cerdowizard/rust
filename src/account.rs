use std::io;
use rand::Rng;
#[derive(Debug)]
#[allow(dead_code)]
enum UserInfoError {
    // You can define different error variants here
    MissingId,
    MissingName,
    MissingEmail,
}
#[allow(dead_code)]
struct ApiResponse{
    code: u32,
    message: String,
}
// TODO: implement User Stuct
struct UserInfo {
    id: String,
    name: String,
    email: String,
}


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

#[allow(dead_code)]
impl UserInfo {
    pub fn create_user_info() -> Result<UserInfo, ApiResponse> {
        println!("Enter username");

        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Error reading username");


        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Error reading username");

        let user_id = utils::gen_id();


        let new_user = UserInfo {
            id: user_id,
            name: username,
            email,
        };
        if new_user.id.is_empty() || new_user.name.is_empty() || new_user.email.is_empty() {
            return Err(
                ApiResponse{
                    code:400, message:"Invalid user".to_string()
                }
            );
        }
        Ok(new_user)
    }
}

// TODO implement Bank Stuct pass in the user id
struct BankInfo {
    user_id: String,
    account_id: String,
    account_name: String,
    account_type: String,
    account_balance: f64,
    can_bollowe: bool,
}

struct LoanInfo {
    user_id: String,
    account_id: String,
    loan_id: String,
    loan_amount: f64,
    loan_intrest: f64,
}
