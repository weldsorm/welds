// Function to split a raw SQL string into separate commands.
// It correctly handles semicolons within quoted strings, ensuring they are not
// treated as command separators. Quoted text, including the quotes themselves,
// is preserved in the output.
pub(crate) fn split_sql_commands(input: &str) -> Vec<String> {
    let mut commands = Vec::new(); // Holds the separated commands
    let mut current_command = String::new(); // Temporary storage for the current command being processed
    let mut in_single_quote = false; // Flag to track if we're inside single quotes
    let mut in_double_quote = false; // Flag to track if we're inside double quotes

    for c in input.chars() {
        match c {
            // Toggle the in_single_quote flag if we encounter a single quote
            // and we're not currently inside double quotes
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current_command.push(c);
            }
            // Toggle the in_double_quote flag if we encounter a double quote
            // and we're not currently inside single quotes
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current_command.push(c);
            }
            // If we encounter a semicolon and we're not inside any quotes,
            // it's a command separator
            ';' if !in_single_quote && !in_double_quote => {
                if !current_command.trim().is_empty() {
                    commands.push(current_command.trim().to_string());
                    current_command.clear();
                }
            }
            // If none of the above, simply add the character to the current command
            _ => current_command.push(c),
        }
    }

    // After the loop, add any remaining command to the commands vector
    if !current_command.trim().is_empty() {
        commands.push(current_command.trim().to_string());
    }

    commands
}

use crate::detect::{ColumnDef, TableDef};

/// returns the columndef matching by its name
pub(crate) fn find_column_or_unwrap<'t>(tabledef: &'t TableDef, name: &str) -> &'t ColumnDef {
    let err = format!("Could not find column '{}' in the database", name);
    tabledef
        .columns()
        .iter()
        .find(|&c| c.name() == name)
        .expect(&err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_sql_commands() {
        let input = "SELECT * FROM users; UPDATE users SET name = 'John; Doe' WHERE id = 1;";
        let commands = split_sql_commands(input);
        assert_eq!(
            commands,
            vec![
                "SELECT * FROM users",
                "UPDATE users SET name = 'John; Doe' WHERE id = 1"
            ]
        );
    }

    #[test]
    fn test_with_double_quotes() {
        let input = "INSERT INTO users (name, email) VALUES (\"Jane; Doe\", 'jane@doe.com'); SELECT * FROM users;";
        let commands = split_sql_commands(input);
        assert_eq!(
            commands,
            vec![
                "INSERT INTO users (name, email) VALUES (\"Jane; Doe\", 'jane@doe.com')",
                "SELECT * FROM users"
            ]
        );
    }

    #[test]
    fn test_with_nested_quotes() {
        // Test SQL command with nested quotes
        let input = "INSERT INTO books (title, description) VALUES ('SQL; The \"Easy\" Way', 'A book for \"beginners\"; covers basics and more.'); SELECT id FROM books;";
        let commands = split_sql_commands(input);
        assert_eq!(
            commands,
            vec![
                "INSERT INTO books (title, description) VALUES ('SQL; The \"Easy\" Way', 'A book for \"beginners\"; covers basics and more.')",
                "SELECT id FROM books"
            ]
        );
    }

    #[test]
    fn test_empty_and_whitespace() {
        // Test input with empty command and commands separated by multiple semicolons with whitespace
        let input = "SELECT * FROM users;;  ; UPDATE users SET active = false WHERE last_login < '2023-01-01';";
        let commands = split_sql_commands(input);
        assert_eq!(
            commands,
            vec![
                "SELECT * FROM users",
                "UPDATE users SET active = false WHERE last_login < '2023-01-01'"
            ]
        );
    }
}
