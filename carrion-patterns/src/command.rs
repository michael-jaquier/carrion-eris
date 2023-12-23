pub struct Command<'a, T> {
    description: &'a str,
    command: &'a str,
    program: fn() -> T,
}

impl<'a, T> Command<'a, T> {
    pub fn new(description: &'a str, command: &'a str, program: fn() -> T) -> Self {
        Self {
            description,
            command,
            program,
        }
    }
    pub fn execute(&self) {
        (self.program)();
    }

    pub const fn new_const(description: &'a str, command: &'a str, program: fn() -> T) -> Self {
        Self {
            description,
            command,
            program,
        }
    }
}

static HELP: Command<()> = Command::new_const("This is a help command", "-h", || println!("Help"));

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_a_command() {
        let h = Command::new("This is a help command", "-h", || println!("Help"));
        h.execute();
        HELP.execute()
    }
}
