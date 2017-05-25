


fn get_css_links(handle: Handle) -> Vec<String> {
    let mut csslinks: Vec<String> = Vec::new();
    let mut queue: Vec<Handle> = Vec::new();
    queue.push(handle);
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle;
        match node.data {
            NodeData::Element{ref name, ref attrs, ..} => {
                assert!(name.ns == ns!(html));
                let mut is_css = false;
                for attr in attrs.borrow().iter() {
                    assert!(attr.name.ns == ns!());
                    let link = string_cache::Atom::from("link");
                    let type = string_cache::Atom::from("type");
                    let css = Tendril::from("text/css");
                    if name.local == link && 
                        attr.name.local == type && 
                        attr.value == css {
                            is_css = true;
                    }
                    let href = string_cache::Atom::from("href");
                    if is_css && attr.name.local == href {
                        let link = String::from(attr.value.clone());
                        csslinks.push(link);
                    }
                }
            }
            _ => {
                //don't do anything
            }
        }
        for child in node.children.borrow().iter() {
            queue.push(child.clone());
        }
    }
    return csslinks;
}