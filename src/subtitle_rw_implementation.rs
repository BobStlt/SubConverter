//mod subtitle_rw_interface;
//use subtitle_rw_interface::subtitle;


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
