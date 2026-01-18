use mac_address::get_mac_address;
use arboard::Clipboard;
use colored::*;
use std::process;

fn main() {
    println!("{}", "=".repeat(50).bright_cyan());
    println!("{}", "           Calyrex - B3n00n - CombaticaLTD".bright_cyan().italic());
    println!("{}", "=".repeat(50).bright_cyan());
    println!();

    let mac = match get_mac_address() {
        Ok(Some(mac)) => mac.to_string(),
        Ok(None) => {
            eprintln!("{}", "Error: No MAC address found".bright_red().bold());
            eprintln!("Press any key to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{} {}", "Error getting MAC address:".bright_red().bold(), e);
            eprintln!("Press any key to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
            process::exit(1);
        }
    };

    println!("{} {}", "MAC Address:".bright_white().bold(), mac.bright_green().bold());
    println!();

    match Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(&mac) {
                Ok(_) => println!("{}", "MAC copied to clipboard successfully".bright_green()),
                Err(e) => {
                    eprintln!("{} {}", "Warning: Failed to copy to clipboard:".yellow(), e);
                    eprintln!("{}", "You can manually copy the MAC address shown above.".yellow());
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Warning: Failed to access clipboard:".yellow(), e);
            eprintln!("{}", "You can manually copy the MAC address shown above.".yellow());
        }
    }

    println!();
    println!("{}", "Press Enter to exit...".bright_black());
    let _ = std::io::stdin().read_line(&mut String::new());
}
