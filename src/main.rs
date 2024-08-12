use std::collections::BTreeMap;


trait Connect {
    fn establish_connection (&self) -> Result<(), String>;
}

trait IO<T> {
    fn write (&self) -> Result<(), String>; 
    fn read  (&self) -> Result<T, String>;
}

trait Prettify {
    fn format_buffer (&self) -> Result<(), String>;
}


struct ComponentProperty {
    component_name: String,
    component_type: String,
    component_access_type: String,
}
struct FileKind {
   property:  ComponentProperty, 
}
struct TerminalKind {
   property:  ComponentProperty, 
}
struct DbKind {
   property:  ComponentProperty, 
}

struct Interfaces {
    files : BTreeMap<String, FileKind>,
    terminals : BTreeMap<String, TerminalKind>,
    databases : BTreeMap<String, DbKind>, 
}

impl Connect for FileKind {
    fn establish_connection (&self) -> Result<(), String> {
        return Ok(());
    }
}

impl Connect for TerminalKind {
    fn establish_connection (&self) -> Result<(), String> {
        return Ok(());
    }
}

impl Connect for DbKind {
    fn establish_connection (&self) -> Result<(), String> {
        return Ok(());
    }
}

fn main() {

}

