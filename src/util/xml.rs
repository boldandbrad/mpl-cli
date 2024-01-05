use xmltree::Element;

pub fn get_child_element_val(element: &Element, child_name: &str) -> String {
    let child_element = element.get_child(child_name);
    match child_element {
        Some(x) => x.attributes.get("value").unwrap().to_string(),
        _ => "0".to_string(),
    }
}
