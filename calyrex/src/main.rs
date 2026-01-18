use arboard::Clipboard;
use colored::*;
use std::process;

fn main() {
    println!("{}", "=".repeat(50).bright_cyan());
    println!("{}", "           Calyrex - B3n00n - CombaticaLTD".bright_cyan().italic());
    println!("{}", "=".repeat(50).bright_cyan());
    println!();

    let machine_id = match machine_uid::get() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{} {}", "Error getting machine ID:".bright_red().bold(), e);
            eprintln!("Press any key to exit...");
            let _ = std::io::stdin().read_line(&mut String::new());
            process::exit(1);
        }
    };

    println!("{} {}", "Machine ID:".bright_white().bold(), machine_id.bright_green().bold());
    println!();

    match Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(&machine_id) {
                Ok(_) => println!("{}", "Machine ID copied to clipboard successfully".bright_green()),
                Err(e) => {
                    eprintln!("{} {}", "Warning: Failed to copy to clipboard:".yellow(), e);
                    eprintln!("{}", "You can manually copy the machine ID shown above.".yellow());
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Warning: Failed to access clipboard:".yellow(), e);
            eprintln!("{}", "You can manually copy the machine ID shown above.".yellow());
        }
    }

    println!();
    println!("{}", "Press Enter to exit...".bright_black());
    let _ = std::io::stdin().read_line(&mut String::new());
}
