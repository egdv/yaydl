// main_no_unsafe

// todo:  7/24
//  done:make sb code pure safe
//  done:compare main.rs then minimize changes
//  done:args as function arg

use crate::definitions;
use crate::download;

// use crate::Args;  // can be used if struct Args from main.rs is made public

pub fn download_with_safe_code(
    handler: <inventory::iter<&dyn definitions::SiteDefinition> as IntoIterator>::Item,
    video_info: &mut String,
    in_url: &str,
    webdriver_port: u16,
    only_audio: bool,
    outputfile: Option<String>,
) {

    if outputfile != None {
        println!("options --output | -o are currently not supported for this handler, using auto generated filename");
    }

    let _b = handler.does_video_exist_s(video_info, in_url, webdriver_port);

    // -> Result<String>
    let url = match handler.find_video_direct_url_s(video_info, in_url, webdriver_port, only_audio)
    {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };

    // -> Result<String>
    let ext = match handler.find_video_file_extension(in_url, webdriver_port, only_audio) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };

    let base_filename = handler.base_filename(in_url);

    let targetfile = format!("{}.{}", base_filename, ext);

    println!("Downloading... to: {}", targetfile);

    // download
    let r = download::download(url.as_str(), targetfile.as_str());

    println!("Result = {:?}\nDone", r);

    // may need to add code from main.rs below this comment
    // Convert the file if needed.

}
