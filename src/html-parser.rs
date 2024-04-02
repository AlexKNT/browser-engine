struct Parser {
    pos: usize,
    input: String,
}


impl Parser {
    
    /// Returns the next character in the input string.
    ///
    /// # Examples
    ///
    /// ```
    /// let parser = Parser { /* ... */ };
    /// let next_char = parser.next_char();
    /// println!("Next character: {}", next_char);
    /// ```
    ///
    /// # Returns
    ///
    /// The next character in the input string.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Checks if the input string starts with the specified string.
    ///
    /// # Parameters
    ///
    /// - `s`: The string to check for.
    ///
    /// # Returns
    ///
    /// `true` if the input string starts with the specified string, `false` otherwise.
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Checks if the parser has reached the end of the input string.
    ///
    /// # Returns
    ///
    /// `true` if the parser has reached the end of the input string, `false` otherwise.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consumes the next character in the input string and advances the parser position.
    ///
    /// # Returns
    ///
    /// The consumed character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Consumes characters from the input string while the specified condition is true.
    ///
    /// # Parameters
    ///
    /// - `test`: A closure that takes a character and returns a boolean value.
    ///
    /// # Returns
    ///
    /// A string containing the consumed characters.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Consumes whitespace characters from the input string.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Parses the tag name from the input string.
    ///
    /// # Returns
    ///
    /// The parsed tag name as a string.
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    /// Parses a node from the input string.
    ///
    /// # Returns
    ///
    /// The parsed node.
    fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// Parses a text node from the input string.
    ///
    /// # Returns
    ///
    /// The parsed text node.
    fn parse_text(&mut self) -> Node {
        text(self.consume_while(|c| c != '<'))
    }

    /// Parses an element node from the input string.
    ///
    /// # Returns
    ///
    /// The parsed element node.
    fn parse_element(&mut self) -> Node {
        // Opening tag
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        // Children
        let children = self.parse_nodes();

        // Closing tag
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        elem(tag_name, attrs, children)
    }

    /// Parses an attribute from the input string.
    ///
    /// # Returns
    ///
    /// A tuple containing the attribute name and value.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        (name, value)
    }

    /// Parses an attribute value from the input string.
    ///
    /// # Returns
    ///
    /// The parsed attribute value as a string.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// Parses attributes from the input string.
    ///
    /// # Returns
    ///
    /// A map containing the parsed attributes.
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    /// Parses nodes from the input string.
    ///
    /// # Returns
    ///
    /// A vector containing the parsed nodes.
    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parses an HTML document from the input string.
    ///
    /// # Parameters
    ///
    /// - `source`: The input string containing the HTML document.
    ///
    /// # Returns
    ///
    /// The parsed HTML document as a `Node`.
    pub fn parse(source: String) -> Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        // If the document contains a single node, return it
        if nodes.len() == 1 {
            nodes.swap_remove(0)
        } else {
            // Otherwise, create a synthetic element to wrap the nodes
            elem("html".to_string(), HashMap::new(), nodes)
        }
    }

    // Parse comment
    fn parse_comment(&mut self) -> Node {
        self.consume_char();
        self.consume_char();
        self.consume_while(|c| c != '-');
        self.consume_char();
        self.consume_char();
        self.consume_char();
        text("".to_string())
    }

    /// Parses a doctype node from the input string.
    ///
    /// # Returns
    ///
    /// The parsed doctype node.
    fn parse_doctype(&mut self) -> Node {
        // Opening tag
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '!');
        assert_eq!(self.consume_char(), 'D');
        assert_eq!(self.consume_char(), 'O');
        assert_eq!(self.consume_char(), 'C');
        assert_eq!(self.consume_char(), 'T');
        assert_eq!(self.consume_char(), 'Y');
        assert_eq!(self.consume_char(), 'P');
        assert_eq!(self.consume_char(), 'E');
        self.consume_while(|c| c != '>');
        assert_eq!(self.consume_char(), '>');

        doctype()
    }

}

