mod pentry;

use pentry::{ServiceInfo, prompt, read_passwords_from_file};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;

fn clr() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("\x1B[H"); // تحريك المؤشر لأعلى بعد المسح
}

fn main() {
    clr();
    let ascii = r#"
__                        __   __            .      
\__   \_    __ __ \   \ /   /____    |  |_/  |_ 
 |     _/\  \  /  _//  _/  \   Y   /\__  \ |  |  \  |\   \
 |    |     /  \_\_ \ \_ \    \     /  /  \|  |  /  |_|  |  
 |__|    (  /  >  >\_/  (__  //|__/|  
                \/     \/     \/_/           \/                 
    "#;

    println!("{ascii}"); 

    loop { println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let choice = prompt("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                entry.write_to_file();
                println!("\n✅ Entry added successfully!\n");
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("❌ Error reading passwords: {}", err);
                    Vec::new()
                });
                
                if services.is_empty() {
                    println!("⚠️ No saved entries found.");
                } else {
                    for item in &services {
                        println!(
                            "\n📌 Service: {}\n👤 Username: {}\n🔑 Password: {}\n",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("❌ Error reading passwords: {}", err);
                    Vec::new()
                });
                let search = prompt("🔍 Search for a service: ");
                let mut found = false;
                
                for item in &services {
                    if item.service.eq_ignore_ascii_case(&search) {
                        println!(
                            "\n📌 Service: {}\n👤 Username: {}\n🔑 Password: {}\n",
                            item.service, item.username, item.password
                        );
                        found = true;
                    }
                }
                
                if !found {
                    println!("⚠️ No matching entry found.");
                }
            }
            "4" => {
                clr();
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice, please enter a number from 1 to 4."),
        }

        println!("\n-----------------------------\n");
    }
}