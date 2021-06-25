#[macro_export]
macro_rules! add_function {
    ($repl: ident, $func_name: expr, $func: ident) => {
        $repl.add_function(
            $func_name.to_string(),
            $func
                as fn(
                    std::collections::HashMap<String, std::sync::Arc<dyn std::any::Any>>,
                    Vec<String>,
                ),
        );
    };
}
