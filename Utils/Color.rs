
#[derive(Debug)]
pub enum Color {
   Red ,
   Green ,
   Blue,
   Yellow,
   White
}


impl Color {

   pub fn colortext(&self , text : &str) {
        let color = match self {
            Color::Red => "\x1b[0;31m",
            Color::Green => "\x1b[0;32m",
            Color::Blue => "\x1b[0;34m",
            Color::Yellow => "\x1b[1;33m",
            Color::White => "\x1b[0;37m"
        };

        let line = format!("{}{}\x1b[0m", color , text);

        println!("{}" , line);

    }

}
