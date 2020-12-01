use std::fs::File;
use std::convert;

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

/* This should not be part of the external interface, this should only be used
 * by implementers of the interface */
pub(super) mod subtitle
{
    use std::rc::Rc;

    /* we wnat to restrict the text lenth so the subs don't
     * take up too much of the screen */
    const MAXSUBLENGTH: usize = 80;

    ///This is the error type for when parsing a new subtitle fails
    #[derive(std::fmt::Debug)]
    pub enum SubTitleError
    {
        Start(String),
        End(String),
        Text(String)
    }

    //when we clone the Rc's will keep the refference of the same strings
    #[derive(Clone, Debug)] 
    pub struct SubTitle
    {
        /* Thease have all been left read only so users of this can access the
         * fields */
        pub start: Rc<String>,
        pub end: Rc<String>,
        pub text: Rc<String>
    }

    impl SubTitle
    {
        /**
         * This creates a new SubTitle as an Ok or returns a SubTitleError with
         * what when wrong
         */
        pub fn new_from_strs(start: &str, end: &str, text: &str)
            -> Result<SubTitle, SubTitleError>
        {
            /* Times should be in the format xx:xx:xx.xx and the text should
             * not be wider than 80 chars */

            //This function check just the time format
            let valid_time_format =
                |to_test: &str| -> bool
                {
                    let time_segments: Vec<&str> = to_test.split(":").collect();

                    //if we have the correct ammount of segments
                    if time_segments.len() == 3
                    {
                        let sec_segments: Vec<&str> = time_segments[2]
                                    .split(".")
                                    .collect();

                        //there should be secconds and millis
                        if sec_segments.len() == 2
                        {
                            let to_check = vec!{time_segments[0],
                                                time_segments[1],
                                                sec_segments[0],
                                                sec_segments[1]};
                            for item in to_check
                            {
                                //Check that we have two digits in each section
                                if item.len() !=2 { return false; }

                                /* Check that all the fields contain digits of
                                 * base 10 */
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

                    /* TODO: change this so it checks weather each line in the
                     * sub title is less than 80 chars */
                    //if text.len() <= MAXSUBLENGTH
                    {
                        //Finally we build our struct and return it
                        Ok(SubTitle
                        {
                            start: Rc::new(String::from(start)),
                            end: Rc::new(String::from(end)),
                            text: Rc::new(String::from(text))
                        })
                    }
                    /* else
                    {
                        let mut  tmp_error_text = String::from(
                            "This subtitle is too long: ");
                        tmp_error_text.push_str(text);
                        Err(SubTitleError::Text(tmp_error_text))
                    }*/
                }
                else
                {
                    let mut  tmp_error_text = String::from(
                        "End has invalid time format: ");
                    tmp_error_text.push_str(end);
                    Err(SubTitleError::End(tmp_error_text))
                }
            }
            else
            {
                let mut  tmp_error_text = String::from(
                    "Start has invalid time format: ");
                tmp_error_text.push_str(start);
                Err(SubTitleError::Start(tmp_error_text))
            }
        }

        pub fn split_time_stamp(stamp: &Rc<String>) -> Vec<&str>
        {
            let mut split_stamp = Vec::new();

            let split_time_stamp = (*stamp)
                        .split(':')
                        .collect::<Vec<&str>>();
            let sec_mili_split = split_time_stamp[2]
                        .split('.')
                        .collect::<Vec<&str>>();
            
            //2 because the last number is the index we break on
            for i in 0..2
            {
                split_stamp.push(split_time_stamp[i]);
            }

            for section in sec_mili_split
            {
                split_stamp.push(section);
            }

            split_stamp
        }
    }
}

/* TRAITS FOR CONVERTERS */

use super::subtitle_rw_interface::subtitle::{*};
use std::io;

/* For when we have an issue reading a subtitle file
 * we can determin weather we had an io error or if
 * something else went wrong */
#[derive(Debug)]
pub enum SubReadError
{
    SubTitleError(String),
    IoError(io::Error)
}

impl convert::From<std::io::Error> for SubReadError
{
    fn from(error: std::io::Error) -> Self
    {
        SubReadError::IoError(error)
    }
}

//Generic IO Error Result
pub type GIOEResult<T> = Result<T, io::Error>;

//SubTitle Reader Result
pub type SRResault<T> = Result<T, SubReadError>;

//Have iterators only return None if we have no file

pub trait SubTitleReader: std::iter::Iterator
{
    fn new(file: File) -> Self where Self: Sized;
    fn set_file(&mut self, file: File);
    fn read_sub(&mut self) -> SRResault<SubTitle>;
}

pub trait SubTitleWriter
{
    fn new(file: File) -> Self where Self: Sized;
    fn set_file(&mut self, file: File);
    fn write_sub(&mut self, to_write: &SubTitle) -> GIOEResult<()>;
}

/* pub trait SubTitleReaderWriter: SubTitleReader + SubTitleWriter
{
    fn new(input_file: File, output_file: File) -> Self where Self: Sized;
    fn set_file(&mut self, file: File);
} */

