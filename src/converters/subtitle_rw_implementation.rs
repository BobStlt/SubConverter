use std::{ rc::Rc, io::{BufReader, BufRead, Write}, fs::File, };
use super::subtitle_rw_interface::{ *, subtitle::{*} };
use regex::Regex;

/* THE CONVERTERS THEMSELVES */

/* Remember intermidiate time format is xx:xx:xx.xx */

struct WebVttReader
{
    /* so if the user calls the iterater after we have
     * had an error or hit the end of file we can return
     * None */
    valid_file: bool,
    /* if we have read the 'WEBVTT' line at the start of the file */
    previously_read: bool, 
    buf_rd: Box<dyn BufRead>,
}

/**
 * Extracts the webVtt time stamps from the given webVtt line, converting them
 * into the universal time stamp format. This ignors any fromatting string after
 * the duration text*/
 //TODO: Deside if you want to keep the formatting string for other puposes
fn collect_timestamps(time_stamp_ln: &str) -> Option<Vec<String>>
{
    let mut split_stamp_ln: Vec<&str> = time_stamp_ln.split(' ').collect();
    let mut ret: Vec<String> = Vec::new();
   
    //deletes the '-->' from our vec leaving us just with the time stamps
    split_stamp_ln.remove(1);

    //will only == 12 if both == 12
    let combined_stamp_len = split_stamp_ln[0].len() & split_stamp_ln[1].len();
    if combined_stamp_len == 12
    {
        for stamp in split_stamp_ln
        {
            let mut tmp_stamp = String::from(stamp);
            /* if its the "end" time stamp it will also be then end of the
             * time stamp line and thus */
             tmp_stamp = tmp_stamp.trim().to_string();
            //drop the last digit of millis
            tmp_stamp.pop();
            ret.push(tmp_stamp);
        }
        Some(ret)
    }
    else
    {
        None
    }
}

impl std::iter::Iterator for WebVttReader
{
    type Item = SubTitle;

    fn next(&mut self) -> Option<Self::Item>
    {
        let potential_subtitle = self.read_sub();
        if potential_subtitle.is_ok()
        {
            Some(potential_subtitle.unwrap())
        }
        else
        {
            None
        }
    }
}

impl SubTitleReader for WebVttReader
{
    //Self is like a third person self
    fn new(file: File) -> Self
    {
        WebVttReader
        {
            valid_file: false,
            previously_read: false,
            buf_rd: Box::new(BufReader::new(file)),
        }
    }

    fn set_file(&mut self, file: File)
    {
        //TODO: close other file ect
        self.valid_file = false;
        self.previously_read = false;
        self.buf_rd = Box::new(BufReader::new(file));
    }

    fn read_sub(&mut self) -> SRResault<SubTitle>
    {
        if (!self.valid_file) && self.previously_read
        {
            Err(SubReadError::SubTitleError("PreviousError".to_string()))
        }
        else
        {
            let mut curr_line = (String::new(), 0);
            if !self.previously_read
            {
                //search for the 'WEBVTT' label at the start
                curr_line.1 = self.buf_rd.read_line(&mut curr_line.0)?;
                self.previously_read = true;
                curr_line.0 = curr_line.0.trim().to_string();
                if curr_line.0 != "WEBVTT"
                {
                    self.valid_file = false;
                    return Err(SubReadError::SubTitleError("No file WEBVTT tag".to_string()));
                }
                else
                {
                    self.valid_file = true;
                }
            }

            //get the next record and return it as a 'SubTile'
            curr_line.0 = String::new();
            curr_line.1 = self.buf_rd.read_line(&mut curr_line.0)?;
            //skip all blank lines
            while curr_line.1 > 0 && curr_line.0.chars().nth(0).unwrap() == '\n'
            {
                curr_line.0 = String::new();
                curr_line.1 = self.buf_rd.read_line(&mut curr_line.0)?;
            }
            
            //if we reach EOF
            if curr_line.1 == 0
            {
                self.valid_file = false;
                Err(SubReadError::SubTitleError("There are no subtitles in this file".to_string()))
            }
            else
            {
                let mut sub_title_lns: Vec<String> = Vec::new();
                //the first line is always the time stamps and we already read it
                sub_title_lns.push(curr_line.0.clone());

                //read until the next blank line
                curr_line.0 = String::new();
                curr_line.1 = self.buf_rd.read_line(&mut curr_line.0)?;
                //while we don't have a blank line or the EOF
                while curr_line.1 >= 2
                {
                    sub_title_lns.push(curr_line.0.clone());
                    curr_line.0 = String::new();
                    curr_line.1 = self.buf_rd.read_line(&mut curr_line.0)?;
                }
                
                //time stamp signiture
                /* TODO: allow any text after the stamp so we can still detect
                 * time stamps that have formatting code after */
                let time_stamp_regex = Regex::new(
                    r"\d{2}:\d{2}:\d{2}.\d{3} --> \d{2}:\d{2}:\d{2}.\d{3}")
                     .unwrap();
                if time_stamp_regex.is_match(&sub_title_lns[0])
                {
                    //get the time stamps from the first line
                    let potential_time_stamps = collect_timestamps(&sub_title_lns[0]);
                    if potential_time_stamps.is_some()
                    {
                        let mut time_stamps = potential_time_stamps.unwrap();

                        //Start building the subtitle
                        let mut tmp_sub_data: Vec<String> = Vec::new();
                        //data needs to be in the format { start, end, text }
                        tmp_sub_data.push(time_stamps.remove(0)); 
                        tmp_sub_data.push(time_stamps.remove(0));

                        //all the rest of the lines in sub_title_lns are our text lines
                        let mut text = String::new();

                        /* TODO: deside if we want to keep the person who
                         * wrote the message in the final subtitle */
                        for (i, line) in sub_title_lns.iter().enumerate()
                        {
                            //replace the new lines that where there with spaces
                            //TODO: if the line is longer than 80 split
                            if i != 0
                            {
                                if i == 1
                                {
                                    text.push_str(&format!("{}", line));
                                }
                                else
                                {
                                    //add a space at the start
                                    text.push_str(&format!(" {}", line));
                                }
                            }
                        }
                        tmp_sub_data.push(text);

                        let potential_subtitle = SubTitle::new_from_strs(&tmp_sub_data[0], &tmp_sub_data[1], &tmp_sub_data[2]);
                        if potential_subtitle.is_ok()
                        {
                            SRResault::Ok(potential_subtitle.unwrap())
                        }
                        else
                        {
                            let error = potential_subtitle.err().unwrap();
                            let mut errormsg = String::new();
                            errormsg.push_str("Could not make subtitle from the data");
                            errormsg.push_str(&format!("{:?}", error));
                            Err(SubReadError::SubTitleError(errormsg))
                        }
                    }
                    else
                    {
                        /*TODO: this does not take into account any formatting code
                         * and comments that WEBVTT can contain*/
                        self.valid_file = false;
                        Err(SubReadError::SubTitleError("Could not collect time stamps".to_string()))
                    }
                }
                else
                {
                    self.valid_file = false;
                    Err(SubReadError::SubTitleError("Missing time stamp line".to_string()))
                }
            
            }
        
        }
    }


}

struct SubRipWriter
{
    file: File,
    last_written: i32
}

impl SubRipWriter
{
    //Converts a subtitle to a vector of all the strings needed to write a subrip record
    fn convert_to_subrip(subtitle: &SubTitle, sub_index: i32) -> Vec<String>
    {
        let convert_time_str = |timestr: &Rc<String>| -> String
        {
            let mut sub_rip_time = String::new();
            let split_tm_stamp = SubTitle::split_time_stamp(timestr);
            let mut split_tm_stamp_iter = split_tm_stamp.iter();
        
            let errormsg = "SubTitle time stamp in unexpectd format";

            sub_rip_time.push_str(*(split_tm_stamp_iter.next().expect(errormsg)));
            sub_rip_time.push(':');
            sub_rip_time.push_str(*(split_tm_stamp_iter.next().expect(errormsg)));
            sub_rip_time.push(':');
            sub_rip_time.push_str(*(split_tm_stamp_iter.next().expect(errormsg)));
            sub_rip_time.push(',');
            sub_rip_time.push_str(*(split_tm_stamp_iter.next().expect(errormsg)));

            sub_rip_time
        };

        let mut ret_sub = Vec::new();
        ret_sub.push(sub_index.to_string());

        let mut timestr = String::new();
        timestr.push_str((convert_time_str(&subtitle.start)).as_str());
        timestr.push_str(" --> ");
        timestr.push_str((convert_time_str(&subtitle.end)).as_str());
        ret_sub.push(timestr);

        //This is a clone of the Rc but its being put into a new string so cloning is fine
        ret_sub.push((*subtitle.text).clone());
        
        ret_sub
    }
}

impl SubTitleWriter for SubRipWriter
{
    fn new(file: File) -> Self
    {
        SubRipWriter
        {
            file: file,
            last_written: 0
        }
    }

    fn set_file(&mut self, file: File)
    {
        self.file = file;
    }

    fn write_sub(&mut self, to_write: &SubTitle) -> GIOEResult<()>
    {
        let converted_sub = SubRipWriter::convert_to_subrip(to_write, self.last_written + 1);

        for sub_line in converted_sub
        {
            self.file.write(sub_line.as_bytes())?;
            //write a new line (were just formatting it as an array)
            //be awear that we don't know weather the file was opend in r/w mode
            self.file.write(&[b'\n';1])?;
        }

        self.last_written += 1;
        Ok(())
    }
}

/* THIS IS A FACTORY METHOD TO BE USED BY MAIN TO GET A READER / WRITER */

/** This takes a file name and optionally an explicit file type
 * and returns a subtitle reader */
pub fn create_sub_reader(file_name: String) -> Result<Box<dyn SubTitleReader<Item = SubTitle>>, String>
{
    //The only conversion we support is from webVtt to subrip so
    //TODO: either add detection logic to the readers or add it here
    Ok(
        Box::new(
            WebVttReader::new(
                match File::open(file_name.clone())
                {
                    Ok(x) => x,
                    Err(er) => return Err(format!("Failed to open {} because {:?}",
                                        file_name, er))
                }
            )
        )
    )
}

pub fn create_sub_writer(file_name: String) -> Result<Box<dyn SubTitleWriter>, String>
{
    //The only conversion we support is from webVtt to subrip so
    //TODO: Figure how to know what type of output the user wanted
    Ok(
        Box::new(
            SubRipWriter::new(
                match File::create(file_name.clone())
                {
                    Ok(x) => x,
                    Err(er) => return Err(format!("Failed to open {} because {:?}",
                                        file_name, er))
                }
            )
        )
    )
}
