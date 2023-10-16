// use windows::Win32::System::Console::GetConsoleCP;
use encoding_rs::{GBK, UTF_8};

pub fn detect_decode(bytes: &[u8]) -> String {
    let (utf8_text, has_errors) = UTF_8.decode_without_bom_handling(bytes);
    if !has_errors {
        return utf8_text.into_owned();
    }

    let (text, has_errors) = GBK.decode_without_bom_handling(bytes);
    if !has_errors {
        return text.into_owned();
    }
    
    utf8_text.into_owned()
}
