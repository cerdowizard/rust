use std::io;

#[derive(Debug)]
#[allow(dead_code)]
enum UserInfoError {
    // You can define different error variants here
    MissingId,
    MissingName,
    MissingEmail,
}
#[allow(dead_code)]
#[derive(Debug)]
struct ApiResponse {
    code: u32,
    message: String,
}
// TODO: implement User Stuct
#[derive(Debug)]
struct UserInfo {
    id: String,
    name: String,
    email: String,
}

#[allow(dead_code)]
impl UserInfo {
    pub fn create_user_info() -> Result<UserInfo, ApiResponse> {
        println!("Enter username");

        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Error reading username");

        println!("Enter Email");
        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("Error reading username");

        let user_id = gen_id();

        let new_user = UserInfo {
            id: user_id.trim().to_string(),
            name: username.trim().to_string(),
            email: email.trim().to_string(),
        };
        if new_user.id.is_empty() || new_user.name.is_empty() || new_user.email.is_empty() {
            return Err(ApiResponse {
                code: 400,
                message: "Invalid user".to_string(),
            });
        }
        Ok(new_user)
    }
}

use rand::Rng;
pub fn gen_id() -> String {
    let uui = rand::thread_rng().gen_range(0..=100000);
    let new_id = uui.to_string();
    new_id
}

// TODO implement Bank Stuct pass in the user id
#[derive(Debug)]
struct BankInfo {
    user_id: String,
    account_id: String,
    account_name: String,
    account_type: String,
    account_balance: f64,
    can_bollow: bool,
}
impl BankInfo {
    fn create_account_info(user_id: String, account_name: String) -> BankInfo {
        BankInfo {
            user_id: user_id.trim().to_string(),
            account_id: gen_id(),
            account_name: account_name.trim().to_string(),
            account_type: "Saving".to_string(),
            account_balance: 3000.00,
            can_bollow: true,
        }
    }
}

struct LoanInfo {
    user_id: String,
    account_id: String,
    loan_id: String,
    loan_amount: f64,
    loan_intrest: f64,
}

fn checker(name: &str, email: &str, database: &Vec<UserInfo>) ->UserInfo {
    let result = database.iter().find(|&user| user.name == name && user.email == email);
    
    // Print all data in the database
    for data in database.iter() {
        println!("{:?}", data);
    }

}

fn menus() {
    let mut user_database:Vec<UserInfo> = Vec::new();
    let mut account_database:Vec<BankInfo> = Vec::new();
    loop {
        println!("1 >>. Create User");
        println!("2 >>. Get Loan Account");
        println!("3 >>. Get Loan Account");
        println!("4 >>. Get Account Information");
        println!("5 >>. Exit");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input");

        let user_input = user_input.trim();

        if user_input == "1" {
            let new_user = UserInfo::create_user_info().unwrap();
            println!("User created: {:?}", new_user);
            let new_account = BankInfo::create_account_info(new_user.id.clone(), new_user.name.clone());
            println!("Account created: {:?}", new_account);
            user_database.push(new_user);
            account_database.push(new_account);
            
        } else if user_input == "2" {
            println!("Please enter your account information");

            println!("Please enter your account Name");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("failed to read line");
            name.trim().to_string();


            println!("Please enter your account Email");
            let mut email = String::new();
            io::stdin().read_line(&mut email).expect("failed to read line");
            email.trim().to_string();

            checker(name, email, &user_database);

            println!("Creating Bank Account...");
        } else if user_input == "3" {
            // Implement logic for creating a loan account
            println!("Creating Loan Account...");
        } else if user_input == "4" {
            break;
        } else {
            println!("Invalid choice. Please enter a valid option.");
        }
    }
}

fn main() {
    menus();
}
