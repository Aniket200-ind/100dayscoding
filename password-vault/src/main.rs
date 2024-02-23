mod lib;

use crate::lib::ServiceInfo;

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clear();
    // generate ascii art to print password manager
    let ascii_art = r#"
        ╦═╗┬ ┬┌─┐┌┬┐┬ ┬  ┌─┐┌─┐┌─┐┌─┐   ┬  ┬┌─┐┬ ┬┬ ┌┬┐
        ╠╦╝│ │└─┐ │ └┬┘  ├─┘├─┤└─┐└─┐───└┐┌┘├─┤│ ││  │
        ╩╚═└─┘└─┘ ┴  ┴   ┴  ┴ ┴└─┘└─┘    └┘ ┴ ┴└─┘┴─┘┴
    "#;

    println!("{}", ascii_art);
    // todo!("Update the feature of where the password which are being stored in json file are being stored in databases and add some more functionality around it. Also write more optimzed code");

    loop {
        println!("Password Manager");
        println!("1. Add a password entry");
        println!("2. View all password entries");
        println!("3. Search for a password entry");
        println!("4. Delete a password entry");
        println!("5. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1=> {
                println!("Add a password entry");
                clear();
                let entry = ServiceInfo::new(
                    ServiceInfo::prompt("Service: "),
                    ServiceInfo::prompt("Username: "),
                    ServiceInfo::prompt("Password: "),
                );

                println!("Entry added successfully");
                entry.write_to_file();
            }
            2=> {
                println!("View all password entries");
                clear();
                let services = ServiceInfo::read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Problem reading passwords from file: {}", err);
                    std::process::exit(1);
                });
                for item in services {
                    println!(
                        "Service = {}
                        - Username: {}
                        - Password: {}
                    ", item.service, item.username, item.password);
                }
            }
            3=> {
                println!("Search for a password entry");
                clear();
                let services = ServiceInfo::read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Problem reading passwords from file: {}", err);
                    std::process::exit(1);
                });

                let search = ServiceInfo::prompt("Search: ");

                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!("\
                        Service = {}\
                        -Username: {}\
                        -Password: {}\
                        ", item.service, item.username, item.password);
                    }
                }
            }
            4=> {
                println!("Delete a password entry");

                println!("Enter the service name to delete: ");
                let mut service_name = String::new();
                std::io::stdin().read_line(&mut service_name).expect("Failed to read service name");
                let service_name = service_name.trim();

                match ServiceInfo::delete_password_entry(service_name){
                    Ok(_) => {
                        println!("Entry deleted successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to delete entry: {}", e);
                    }
                }
            }
            5=> {
                clear();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
        println!("\n\n");
    }
}
