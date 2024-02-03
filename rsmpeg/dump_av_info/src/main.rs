use std::error::Error;
use std::ffi::{CStr, CString};
use rsmpeg::avformat::AVFormatContextInput;

fn dump_av_info(path: &CStr) -> Result<(), Box<dyn Error>> {
    let mut input_format_context = AVFormatContextInput::open(path, None, &mut None)?;
    input_format_context.dump(0, path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let cstr = CString::new("https://vcup.moe/Share/SharpMusic.DllHellPTests/SoundSourceTests%600.flac")?;
    dump_av_info(&cstr)?;
    Ok(())
}
