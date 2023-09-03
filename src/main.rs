use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    ExecutableCommand,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    tty::IsTty,
};
use std::io::{self, Read, Write};

struct UI {
    stdout: std::io::Stdout,
    piped_input: String,
    user_input: String,
}

impl UI {
    fn new() -> Self {
        let stdout = io::stdout();
        let piped_input = Self::get_piped_input();
        Self {
            stdout,
            piped_input,
            user_input: String::new(),
        }
    }

    fn get_piped_input() -> String {
        let mut piped_input = String::new();
        if !io::stdin().is_tty() {
            let _ = io::stdin().read_to_string(&mut piped_input);
        }
        piped_input
    }

    fn setup(&mut self) -> io::Result<()> {
        execute!(self.stdout, EnterAlternateScreen, MoveTo(0, 0), Hide)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    fn teardown(&mut self) -> io::Result<()> {
        execute!(self.stdout, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn display_prompt(&mut self) -> io::Result<()> {
        let prompt = "Enter your input: ";
        self.stdout.execute(Print(prompt))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn execute_input(&mut self) -> io::Result<()> {
        self.stdout.execute(SetForegroundColor(Color::Green))?;
        write!(self.stdout, "\r\nYou entered: {}\r\n", self.user_input)?;
        self.stdout.execute(SetForegroundColor(Color::Reset))?;
        self.stdout.flush()?;
        Ok(())
    }

    fn check_piped_input(&mut self) -> io::Result<()> {
        if !self.piped_input.is_empty() {
            self.stdout.execute(SetForegroundColor(Color::Blue))?;
            write!(self.stdout, "{}", &self.piped_input)?;
            self.stdout.flush()?;
            self.stdout.execute(SetForegroundColor(Color::Reset))?;
            self.user_input = self.piped_input.clone();
            self.piped_input.clear();
        }
        Ok(())
    }

    fn run(&mut self) -> io::Result<()> {
        let mut exit_flag = false;

        loop {
            self.display_prompt()?;
            self.check_piped_input()?;

            if !self.user_input.is_empty() {
                self.execute_input()?;
                self.user_input.clear();
                continue;
            }

            loop {
                if let event::Event::Key(key_event) = event::read()? {
                    match key_event {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            exit_flag = true;
                            break;
                        }
                        KeyEvent {
                            code: KeyCode::Backspace,
                            ..
                        } => {
                            if !self.user_input.is_empty() {
                                self.user_input.pop();
                                self.stdout.execute(Print("\u{8} \u{8}"))?; // Handle backspace
                            }
                        }
                        KeyEvent {
                            code: KeyCode::Char(c),
                            ..
                        } => {
                            self.user_input.push(c);
                            self.stdout.execute(SetForegroundColor(Color::Blue))?;
                            write!(self.stdout, "{}", c)?;
                            self.stdout.flush()?;
                        }
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => {
                            if self.user_input.trim() == "exit" || self.user_input.trim() == "quit" {
                                exit_flag = true;
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if exit_flag {
                break;
            }

            self.execute_input()?;
            self.user_input.clear();
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut ui = UI::new();
    ui.setup()?;
    ui.run()?;
    ui.teardown()?;
    Ok(())
}
