pub trait MessageWriter {
    fn write(&self, message: &str);
}

pub struct ConsoleMessageWriter;

impl MessageWriter for ConsoleMessageWriter {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct Salutation<'a> {
    writer: &'a dyn MessageWriter,
}

impl<'a> Salutation<'a> {
    fn exclaim(&self, message: &str) {
        self.writer.write(message);
    }
}

fn main() {
    let writer = ConsoleMessageWriter;
    let salutation = Salutation { writer: &writer,};

    salutation.exclaim("Hello DI!");
}

