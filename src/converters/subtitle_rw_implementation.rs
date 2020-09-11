use std::{ rc::Rc, io::{Read, Write}, fs::File };
use super::subtitle_rw_interface::{ *, subtitle::{*} };

/* THE CONVERTERS THEMSELVES */

/* Remember times are in the format xx:xx:xx.xx */

struct GenericSubtitleReaderWriter
{
    file: File,
    subtitles: Vec<SubTitle>
}

//we can reuse the struct with different impls with aliases
pub type WebVttReader = GenericSubtitleReaderWriter;

impl std::iter::Iterator for WebVttReader
{
    type Item = SubTitle;

    fn next(&mut self) -> Self::Item
    {
        //TODO
    }
}

impl SubTitleReader for WebVttReader
{
    fn new(file: File) -> Self
    {
        //Self is like a third person self
        Self
        {
            file: file,
            subtitles: Vec::new()
        }
    }

    fn set_file(&self, file: File)
    {
        self.file = file;
    }

    fn read_sub(&self) -> SRResault<()>
    {
        //TODO
    }
}


struct SubRipWriter
{
    writer_dat: GenericSubtitleReaderWriter,
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
            let mut split_tm_stamp = SubTitle::split_time_stamp(timestr).iter();
        
            let errormsg = "SubTitle time stamp in unexpectd format";

            sub_rip_time.push_str(*(split_tm_stamp.next().expect(errormsg)));
            sub_rip_time.push(':');
            sub_rip_time.push_str(*(split_tm_stamp.next().expect(errormsg)));
            sub_rip_time.push(':');
            sub_rip_time.push_str(*(split_tm_stamp.next().expect(errormsg)));
            sub_rip_time.push(',');
            sub_rip_time.push_str(*(split_tm_stamp.next().expect(errormsg)));

            sub_rip_time
        };

        let mut ret_sub = Vec::new();
        ret_sub.push(sub_index.to_string());

        let mut timestr = String::new();
        timestr.push_str((convert_time_str(&subtitle.start)).as_str());
        timestr.push_str(" --> ");
        timestr.push_str((convert_time_str(&subtitle.end)).as_str());
        ret_sub.push(timestr);

        ret_sub.push(*subtitle.text);
        
        ret_sub
    }
}

impl SubTitleWriter for SubRipWriter
{
    fn new(file: File) -> Self
    {
        Self
        {
            writer_dat: GenericSubtitleReaderWriter
                {
                    file: file,
                    subtitles: Vec::new()
                },
            last_written: 0
        }
    }

    fn set_file(&self, file: File)
    {
        self.writer_dat.file = file;
    }

    fn write_sub(&self, to_write: &SubTitle) -> GIOEResult<()>
    {
        let converted_sub = SubRipWriter::convert_to_subrip(to_write, self.last_written + 1);

        for sub_line in converted_sub
        {
            self.writer_dat.file.write(sub_line.as_bytes())?;
            self.writer_dat.file.write(&[b'\n';1])?;
        }

        self.last_written += 1;
        Ok(())
    }
}

/* THIS IS A FACTORY METHOD TO BE USED BY MAIN TO GET A READER / WRITER */

/* * This takes a file name and optionally an explicit file type
 * and returns a subtitle reader */

//TODO



/* * This takes a file name and optionally an explicit file type
 * and return a subtitle writer */

//TODO

