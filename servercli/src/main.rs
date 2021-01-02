use std::io::{Result};
use httpserver;
use std::collections::HashMap;
macro_rules! map(
    ( $( $k:expr => $v:expr ),+ $(,)? ) => ( // $(,)? is to always allow trailing commas
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )+
            map
        }
    );
);

fn main() -> Result<()> {
    let files : HashMap<String,String> = map![
    "/".into() => "index.html".into(),
    "/file".into() => "file.html".into(),
    ];
    httpserver::start(files)?;
    Ok(())
}
