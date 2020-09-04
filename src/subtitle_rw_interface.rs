use std::fs::File;

/* have a sturct that holds all the data in seperate fields,
 * have this be your generic format.
 * 
 * make a trate for a 'sub title reader' then make an implementation for
 * a chat comment reader.
 * then have another trate for a 'sub titles writer' that takes a reader
 * gets the generic format for each chat comment from an terator and
 * writes it out to given file pointer or rusts equivalent
 *
 * Play with using anonumus functions to deal with retries when handling errors
 * and combinators when dealing with generall errors
 */

/* THIS IS THE INTERMIDIATE FORM FOR SUBTITLES */

mod subtitle
{
    use std::rc::Rc;

    /* we wnat to restrict the text lenth so the subs don't
     * take up too much of the screen */
    const MAXSUBLENGTH: usize = 80;

    ///This is the error type for when parsing a new subtitle fails
    pub enum SubTitleError
    {
        Start(String),
        End(String),
        Text(String)
    }

    #[derive(Clone)] //when we clone the Rc's will keep the refference of the same strings
    pub struct SubTitle
    {
        //Thease have all been left read only so users of this can access the fields
        start: Rc<String>,
        end: Rc<String>,
        text: Rc<String>
    }

    impl SubTitle
    {
        ///This creates a new SubTitle as an Ok or returns a SubTitleError with what when wrong
        pub fn new_from_strs(start: &str, end: &str, text: &str) -> Result<SubTitle, SubTitleError>
        {
            /* Times should be in the format xx:xx:xx.xx and the text should not be longer than 80
             * chars */

            //This function check just the time format
            let valid_time_format =
                |to_test: &str| -> bool
                {
                    let time_segments: Vec<&str> = to_test.split(":").collect();

                    //if we have the correct ammount of segments
                    if time_segments.len() == 3
                    {
                        let sec_segments: Vec<&str> = time_segments[2].split(".").collect();

                        //there should be secconds and millis
                        if sec_segments.len() == 2
                        {
                            let to_check = vec!{time_segments[0], time_segments[1], sec_segments[0], sec_segments[1]};
                            for item in to_check
                            {
                                //Check that we have two digits in each section
                                if item.len() !=2 { return false; }

                                //Check that all the fields contain digits of base 10
                                for carac in item.chars()
                                {
                                    if !carac.is_digit(10) { return false; }
                                }
                            }
                            true
                            
                        }
                        else
                        {
                            false
                        }
                    }
                    else
                    {
                        false
                    }
                };

            //We test thease sepperately so we know which stage we get up to
            if valid_time_format(start)
            {
                if valid_time_format(end)
                {

                    if text.len() <= MAXSUBLENGTH
                    {
                        //Finally we build our struct and return it
                        Ok(SubTitle
                        {
                            start: Rc::new(String::from(start)),
                            end: Rc::new(String::from(end)),
                            text: Rc::new(String::from(text))
                        })
                    }
                    else
                    {
                        let mut  tmp_error_text = String::from("This subtitle is too long: ");
                        tmp_error_text.push_str(text);
                        Err(SubTitleError::Text(tmp_error_text))
                    }
                }
                else
                {
                    let mut  tmp_error_text = String::from("End has invalid time format: ");
                    tmp_error_text.push_str(end);
                    Err(SubTitleError::End(tmp_error_text))
                }
            }
            else
            {
                let mut  tmp_error_text = String::from("Start has invalid time format: ");
                tmp_error_text.push_str(start);
                Err(SubTitleError::Start(tmp_error_text))
            }
        }
    }
}

/* TRAITS FOR CONVERTERS */

use crate::converter::subtitle::{*};
use std::io;

/* For when we have an issue reading a subtitle file
 * we can determin weather we had an io error or if
 * something else went wrong */
#[derive(Debug)]
enum SubReadError
{
    SubTitleError(String),
    IoError(io::Error)
}

//Generic IO Error Result
type GIOEResult<T> = Result<T, io::Error>;

//SubTitle Reader Result
type SRResault<T> = Result<T, SubReadError>;

//Have iterators only return None if we have no file

trait SubTitleReader: std::iter::Iterator
{
    fn new(file: File) -> Self;
    fn set_file(&self, file: File) -> GIOEResult<()>;
    fn read_sub(&self) -> SRResault<()>;
}

trait SubTitleWriter: std::iter::Iterator
{
    fn new(file: File) -> Self;
    fn set_file(&self, file: File) -> GIOEResult<()>;
    fn write_sub(&self, to_write: &SubTitle) -> GIOEResult<()>;
}

trait SubTitleReaderWriter: SubTitleReader + SubTitleWriter
{
    fn new(Input_file: File, Output_file: File) -> Self;
}

/* THE CONVERTERS THEMSELVES */
/* TODO: Think about moving this to a new file and renaming
 * this file to converter interface */

struct GenericSubtitleReader
{
    //TODO
}

//we can reuse the struct with different impls with aliases
pub type ColaborateReader = GenericSubtitleReader;

impl std::iter::Iterator for ColaborateReader
{
    //TODO
}

impl SubTitleReader for ColaborateReader
{
    //TODO
}