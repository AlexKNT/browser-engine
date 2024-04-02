struct Stylesheet {
    rules: Vec<Rule>,
    comments: Vec<String>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    BackgroundColor(Color),
    FontSize(f32, Unit),
    FontWeight(f32),
    TextAlign(TextAlignment),
    Display(DisplayType),
    BorderWidth(f32, Unit),
    BorderRadius(f32, Unit),
    BoxShadow(Shadow),
    Opacity(f32),
    Transition(TransitionProperty),
}

enum Unit {
    Px,
    Pt,
    Em,
    Percent,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

enum TextAlignment {
    Left,
    Right,
    Center,
    Justify,
}

enum DisplayType {
    Block,
    Inline,
    None,
}

struct Shadow {
    x: f32,
    y: f32,
    blur: f32,
    spread: f32,
    color: Color,
}

enum TransitionProperty {
    All,
    BackgroundColor,
    BorderColor,
    BorderRadius,
    BorderWidth,
    BoxShadow,
    Color,
    Display,
    FontSize,
    FontWeight,
    Opacity,
    TextAlign,
}

enum AtRule {
    Import(String),
    Media(Vec<MediaQuery>, Vec<Rule>),
    FontFace(Vec<Declaration>),
    Keyframes(String, Vec<Keyframe>),
    Supports(Vec<SupportsCondition>, Vec<Rule>),
    Namespace(String),
    Page(Vec<Declaration>),
    Viewport(Vec<Declaration>),
    CounterStyle(Vec<Declaration>),
    FontFeatureValues(Vec<Declaration>),
    Property(Vec<Declaration>),
    Unknown(String, Vec<Declaration>),
}

struct MediaQuery {
    media_type: Option<String>,
    expressions: Vec<MediaExpression>,
}

struct MediaExpression {
    feature: String,
    value: Option<String>,
}

struct Keyframe {
    selectors: Vec<Percentage>,
    declarations: Vec<Declaration>,
}

enum Percentage {
    Percentage(f32),
}

struct SupportsCondition {
    feature: String,
    value: Option<String>,
}

enum Declaration {
    NameValue(String, Value),
    Unknown(String, Value),
}

impl Stylesheet {
    fn parse(source: &str) -> Stylesheet {
        let mut parser = Parser { pos: 0, input: source };
        Stylesheet { rules: parser.parse_rules(), comments: parser.comments }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for rule in &self.rules {
            result.push_str(&rule.to_string());
        }
        result
    }

    fn to_string_with_comments(&self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str(&format!("/*{}*/\n", comment));
        }
        result.push_str(&self.to_string());
        result
    }

    fn to_string_minified(&self) -> String {
        let mut result = String::new();
        for rule in &self.rules {
            result.push_str(&rule.to_string_minified());
        }
        result
    }

    fn to_string_minified_with_comments(&self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str(&format!("/*{}*/\n", comment));
        }
        result.push_str(&self.to_string_minified());
        result
    }

    fn to_string_with_at_rules(&self) -> String {
        let mut result = String::new();
        for rule in &self.rules {
            result.push_str(&rule.to_string_with_at_rules());
        }
        result
    }

    fn to_string_minified_with_at_rules(&self) -> String {
        let mut result = String::new();
        for rule in &self.rules {
            result.push_str(&rule.to_string_minified_with_at_rules());
        }
        result
    }

    fn to_string_with_comments_and_at_rules(&self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str(&format!("/*{}*/\n", comment));
        }
        result.push_str(&self.to_string_with_at_rules());
        result
    }

    fn to_string_minified_with_comments_and_at_rules(&self) -> String {
        let mut result = String::new();
        for comment in &self.comments {
            result.push_str(&format!("/*{}*/\n", comment));
        }
        result.push_str(&self.to_string_minified_with_at_rules());
        result
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identifier();
        self.consume_whitespace();
        self.consume_char();
        self.consume_whitespace();
        let value = self.parse_value();
        Declaration { name, value }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            'b' => self.parse_keyword(),
            'c' => self.parse_keyword(),
            'd' => self.parse_keyword(),
            'f' => self.parse_keyword(),
            'l' => self.parse_keyword(),
            'o' => self.parse_keyword(),
            'r' => self.parse_keyword(),
            't' => self.parse_keyword(),
            'u' => self.parse_keyword(),
            'v' => self.parse_keyword(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_color(&mut self) -> Value {
        self.consume_char();
        let r = self.parse_hex_pair();
        let g = self.parse_hex_pair();
        let b = self.parse_hex_pair();
        let a = if self.next_char() == ';' {
            255
        } else {
            self.parse_hex_pair()
        };
        Value::ColorValue(Color { r, g, b, a })
    }

    fn parse_keyword(&mut self) -> Value {
        Value::Keyword(self.parse_identifier())
    }

    fn parse_unit(&mut self) -> Unit {
        match self.parse_identifier().as_str() {
            "px" => Unit::Px,
            "pt" => Unit::Pt,
            "em" => Unit::Em,
            "%" => Unit::Percent,
            _ => panic!("Unexpected unit"),
        }
    }

    fn parse_float(&mut self) -> f32 {
        self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        }).parse().unwrap()
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) where F: Fn(char) -> bool {
        while !self.eof() && test(self.next_char()) {
            self.consume_char();
        }
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        while !self.eof() {
            self.consume_whitespace();
            if self.starts_with("/*") {
                self.comments.push(self.parse_comment());
            } else if self.starts_with("@") {
                rules.push(self.parse_at_rule());
            } else {
                rules.push(self.parse_rule());
            }
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

}

pub type Specificity = (usize, usize, usize);

impl Selector {
    fn specificity(&self) -> Specificity {
        let SimpleSelector { id, class, tag_name } = self;
        let a = id.iter().count();
        let b = class.len();
        let c = tag_name.iter().count();
        (a, b, c)
    }
}