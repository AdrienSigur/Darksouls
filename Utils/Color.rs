
#[derive(Debug)]
pub enum color {
   red ,
   green ,
   blue,
   yellow
}


impl color {

   pub fn colortext(&self , text : &str) {
        let color = match self {
            color::red => "\x1b[0;31m",
            color::green => "\x1b[0;32m",
            color::blue => "\x1b[0;34m",
            color::yellow => "\x1b[1;33m"
        };

        let line = format!("{}{}\x1b[0m", color , text);

        println!("{}" , line);

    }

}
