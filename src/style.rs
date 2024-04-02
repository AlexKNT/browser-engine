
type PropertyMap = HashMap<String, String>;


struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple) => matches_simple_selector(element, simple),
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&str> {
        self.attrs.get("id").map(|id| id.as_str())
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.classes().contains(&class)
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|attr| attr.as_str())
    }

    pub fn get_style(&self, key: &str) -> Option<&str> {
        self.styles.get(key).map(|style| style.as_str())
    }

    pub fn get_style_or_default(&self, key: &str, default: &str) -> String {
        self.get_style(key).unwrap_or(default).to_string()
    }

    pub fn get_style_or_inherit(&self, key: &str, parent: &ElementData) -> String {
        self.get_style(key).unwrap_or_else(|| parent.get_style_or_inherit(key, parent))
    }

    pub fn get_style_or_initial(&self, key: &str) -> String {
        self.get_style(key).unwrap_or_else(|| initial_style(key))
    }

    pub fn get_style_or_default_or_inherit(&self, key: &str, default: &str, parent: &ElementData) -> String {
        self.get_style(key).unwrap_or_else(|| parent.get_style_or_default(key, default))
    }

    pub fn get_style_or_default_or_initial(&self, key: &str, default: &str) -> String {
        self.get_style(key).unwrap_or_else(|| initial_style(key))
    }
}

fn matches_simple_selector(element: &ElementData, selector: &SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| element.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| element.id() != Some(id)) {
        return false;
    }

    let element_classes = element.classes();
    if selector.class.iter().any(|class| !element_classes.contains(class)) {
        return false;
    }

    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);


fn match_rule<'a>(element: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter().find(|selector| matches(element, *selector)).map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(element: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(element, rule)).collect()
}

fn specified_values(element: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(element, stylesheet);
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for &(_, rule) in rules.iter() {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        styles: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            _ => HashMap::new(),
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}
