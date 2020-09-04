use super::subtitle_rw_interface::{*};

/* THE CONVERTERS THEMSELVES */

struct GenericSubtitleReader
{
    file: File,
    subtitles: Vec<SubTile>
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

/* THIS IS A FACTORY METHOD TO BE USED BY MAIN TO GET A READER / WRITER */

/** This takes a file name and optionally an explicit file type
 * and returns a subtitle reader */

//TODO



/** This takes a file name and optionally an explicit file type
 * and return a subtitle writer */

//TODO
