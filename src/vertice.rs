/*- Imports -*/
use std::fmt;
use std::io::{ Write, stdout };

/*- Structs, enums & unions -*/
pub(crate) struct Vertice(pub f32, pub f32, pub f32);

/*- Implementations -*/
impl fmt::Debug for Vertice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        stdout().flush().unwrap_or(());
        f.write_fmt(
            format_args!("Vertice({}, {}, {})", self.0, self.1, self.2)
        ).unwrap_or(());
        Ok(())
    }
}