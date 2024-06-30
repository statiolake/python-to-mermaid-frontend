use python_to_mermaid::convert::{flowchart_to_mermaid, python_to_flowchart};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Diagram {
    pub name: String,
    pub diagram: String,
}

#[wasm_bindgen]
pub fn convert(source: &str) -> Result<Vec<Diagram>, String> {
    let fn_defs = python_to_flowchart::enumerate_fn_defs(source)
        .map_err(|e| format!("Parse Error: {}", e))?;

    fn_defs
        .into_iter()
        .map(|f| {
            let name = f.name.clone();
            let diagram = python_to_flowchart::convert(f)
                .map(|fc| flowchart_to_mermaid::convert(&fc))
                .map(|mfc| {
                    let mut s = String::new();
                    mfc.render(&mut s);
                    s
                })
                .map_err(|e| format!("Conversion Error: {}", e))?;

            Ok(Diagram { name, diagram })
        })
        .collect()
}
