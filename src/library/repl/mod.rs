//import SourceCharecter from lexers base
use crate::lexers::{read, read_all, ColumnNumber, LineNumber, ReadingStatus, SourceCharecter};

use std::{
    io::{self, BufRead, BufReader, Read, Write},
    usize,
};

struct BufferedReaderLines<R: Read> {
    reader: BufReader<R>,
    position: usize,
}
impl<R: Read> BufferedReaderLines<R> {
    fn new(reader: BufReader<R>) -> Self {
        BufferedReaderLines {
            reader,
            position: 0,
        }
    }
}
//having BufferedReaderLines implement Iterator return iterator which will return list of lines
//with line number
impl<R: Read> Iterator for BufferedReaderLines<R> {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        self.position += 1;
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some((self.position, line)),
            Err(e) => panic!("Error reading line: {}", e),
        }
    }
}

pub fn start<R, W>(input: &mut R, output: &mut W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    let buffered = BufReader::new(input);
    let mut line_count: usize = 0;
    let readData = buffered.lines().flat_map(move |line| {
        line_count += 1;
        let line_number = LineNumber(line_count as u16);
        let aaa = line
            .unwrap()
            .chars()
            .map(move |ch| {
                let column_number = ColumnNumber(line_count as u16);
                return SourceCharecter::new(ch, column_number, line_number);
            })
            .collect::<Vec<SourceCharecter>>();
        return aaa.into_iter();
    });
    read_all(readData).into_iter().for_each(|token| {
        let line = format!(
            "Line: {}, Column: {}, Token: {:?}",
            token.0 .0, token.1 .0, token.2
        );
        output.write_all(line.as_bytes()).unwrap();
    });

    // fun_name(readData, output);

    Ok(())
}

fn fun_name<R, W>(readData: R, output: &W)
where
    R: Iterator<Item = SourceCharecter>,
    W: Write,
{
    read(readData, |status| match status {
        ReadingStatus::Read(new_tokens) => {
            new_tokens.iter().for_each(|token| {
                let value = format!(
                    "Line: {}, Column: {}, Token: {:?}",
                    token.0 .0, token.1 .0, token.2
                );
                println!("{}", value);
                // output.write_all(value.as_bytes()).unwrap();
            });
        }
        ReadingStatus::Finished => {}
    });
}
