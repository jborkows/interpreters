//import SourceCharecter from lexers base
use crate::lexers::{read_all, SourceCharecter};

use std::{
    io::{self, BufRead, BufReader, Read, Write},
    usize,
};

pub fn start<R, W>(input: &mut R, output: &mut W) -> io::Result<()>
where
    R: Read,
    W: Write,
{
    let buffered = BufReader::new(input);
    let mut line_count: usize = 0;
    let read_data = buffered.lines().flat_map(move |line| {
        line_count += 1;
        let line_number = line_count.into();
        let aaa = line
            .unwrap()
            .chars()
            .map(move |ch| {
                let column_number = line_count.into();
                SourceCharecter::new(ch, column_number, line_number)
            })
            .collect::<Vec<SourceCharecter>>();
        aaa.into_iter()
    });

    read_all(read_data).for_each(|token| {
        let line = format!(
            "=> Line: {}, Column: {}, Token: {:?} \n",
            token.0 .0, token.1 .0, token.2
        );
        output.write_all(line.as_bytes()).unwrap();
    });

    Ok(())
}
