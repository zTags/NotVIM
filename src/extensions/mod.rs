// demo extension
// ```rs
// extension.rs:
// fn on_file_open(name: &str) {
//     println!(name);	
// }
// let extension = Extension {
//     on_file_open 	
// }
// ```
// THIS IS PROBABLY GOING TO GET CHANGED.
pub struct Extension {
	on_file_open: Fn(&str),
}