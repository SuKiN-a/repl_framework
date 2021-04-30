#[macro_export]
macro_rules! add_function {
    ($repl: ident, $func_name: expr, $func: ident) => {
        use std::{any::Any, collections::HashMap, sync::Arc};
        $repl.add_function(
            $func_name.to_string(),
            $func as fn(HashMap<String, Arc<dyn Any>>, Vec<String>),
        );
    };
}
