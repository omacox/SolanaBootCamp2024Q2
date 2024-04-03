// enums3.rs
// Address all the TODOs to make the tests pass!


enum Message {
    Point{x: u8, y: u8},
    State{color:(u8, u8, u8), position: Point,
    quit: bool},
    Quit,
    ChangeColor{x: u8, y: u8,z: u8},
    Echo(String),
    Move(Point),


    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message {
            Message::ChangeColor { x, y, z } => {
                self.change_color((x, y, z));
            }
            Message::Echo(s) => {
                self.echo(s);
            }
            Message::Move(p) => {
                self.move_position(p);
            }
            Message::Quit => {
                self.quit();
            }
            Message::Point { x, y } => {
                // Here you might want to update the state's position with x and y
                // Or log a message, or whatever is appropriate for handling a Point message
                // For example:
                println!("Received Point message with x: {}, y: {}", x, y);
            }
            Message::State { color, position, quit } => {
                // Here you might want to update the state entirely with the provided values
                // Or perform some validation, logging, etc.
                // For example:
                self.color = color;
                self.position = position;
                self.quit = quit;
                println!("Received State message to update state.");
            }
        }
    }


    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };
        state.process(Message::ChangeColor{x: 255,y: 0,z: 255});
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
