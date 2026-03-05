use std::path::Path;
use wm_script::script_converter::convert_script_to_lua;

fn main() {
    let p = Path::new(r"d:\Projects\WMaster\WMRenewal\WhoreMasterRenewal\Resources\Scripts\MeetTownDefault.script");
    match convert_script_to_lua(p) {
        Ok(lua) => println!("{}", lua),
        Err(e) => eprintln!("Error: {}", e),
    }
}
