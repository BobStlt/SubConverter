mod converters;
use crate::converters::subtitle_rw;
use std::env;

/* TODO: When this gets expanded to support more formats we need to create the
 * factory method that determines what the input file type is */

fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() == 3
    {
        //Again at the moment we only support WEBVTT to SUBRIP
        match subtitle_rw::create_sub_reader(args[1].clone()) 
        {
            Ok(sub_reader) =>
            {
                match subtitle_rw::create_sub_writer(args[2].clone())
                {                
                    Ok(mut sub_writer) => 
                    {
                        for subtitle in sub_reader
                        {
                            //TODO: this and the expect is just for testing
                            println!("We have: {:?}",subtitle);
                            sub_writer.write_sub(&subtitle).expect("Faled to write subs");
                        }
                    }
                    Err(er) => eprintln!("Could not open file because: {:?}", er)
                };
            }
            Err(er) =>
            {
                eprintln!("Could not open file because: {:?}", er)
            }
        }
    }
    else
    {
        eprintln!("I need two args, one for the input file and one for the out");
    }

}
