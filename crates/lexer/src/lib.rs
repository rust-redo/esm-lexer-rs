use serde::{Serialize};

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParsedData {
    pub facade: bool,
    pub has_module_syntax: bool
}

impl ParsedData  {
   pub fn new() -> Self {
       Self {
          ..Self::default() 
       }
   }
}


