use xmltree::{Element, XMLNode};

pub fn get_child_element_val(element: &Element, child_name: &str) -> String {
    let child_element = element.get_child(child_name);
    match child_element {
        Some(x) => x.attributes.get("value").unwrap().to_string(),
        _ => "0".to_string(),
    }
}

pub fn get_element_ids(elements: &Vec<XMLNode>) -> Vec<String> {
    let mut element_ids = vec![];
    for child in elements {
        element_ids.push(
            child
                .as_element()
                .unwrap()
                .attributes
                .get("id")
                .unwrap()
                .to_string(),
        )
    }
    element_ids
}
