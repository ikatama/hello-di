pub trait MessageWriter {
    fn write(&self);
}

pub struct ConsoleMessageWriter {
    message: String,
}

impl MessageWriter for ConsoleMessageWriter {
    fn write(&self) {
        println!("{}", self.message);
    }
}

pub struct Salutation<'a> {
    writer: &'a dyn MessageWriter,
}

impl<'a> Salutation<'a> {
    fn exclaim(&self) {
        self.writer.write();
    }
}

fn main() {
    let writer = ConsoleMessageWriter{message: String::from("Hello DI")};
    let salutation = Salutation { writer: &writer,};

    salutation.exclaim();
}

