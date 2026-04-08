pub struct Book {
    pub title: String,
    pub author: String,
    pub date_read: String,
    pub rating: f32,
}

impl Book {
    pub fn read(title: &str, author: &str, date_read: &str, rating: f32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            date_read: date_read.to_string(),
            rating,
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<li>"); 
        html.push_str("<div class=\"book\">\n");
        html.push_str("<div class=\"title\">");
        html.push_str(&self.title);
        html.push_str("</div>\n");
        html.push_str("<div class=\"author\">");
        html.push_str(&self.author);
        html.push_str("</div>\n");
        html.push_str("<div class=\"date-read\">");
        html.push_str(&self.date_read);
        html.push_str("</div>\n");
        html.push_str("<div class=\"rating\">");
        html.push_str(&self.rating.to_string());
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html.push_str("</li>\n"); 
        html
    }
}

pub fn reading_content() -> String {
    let mut html = String::new();
    html.push_str("<ul class=\"reading-list\">\n");
    let mut book = Book::read("The Three Stigmata of Palmer Eldritch", "Philip K. Dick", "2025-07", 4.5).to_html(); 
    html.push_str(book.as_str()); 
    book = Book::read("Normal People", "Sally Rooney", "2025-01", 4.0).to_html(); 
    html.push_str(book.as_str()); 
    book = Book::read("Intermezzo", "Sally Rooney", "2025-01", 4.0).to_html(); 
    html.push_str(book.as_str());
    book = Book::read("Klara and the Sun", "Kazuo Ishiguro", "2025-01", 4.7).to_html(); 
    html.push_str(book.as_str());
    html.push_str("</ul>\n");
    html
}

pub struct Quote {
    pub text: String,
    pub source: String,
}

impl Quote {
    pub fn new(text: &str, source: &str) -> Self {
        Self {
            text: text.to_string(),
            source: source.to_string(),
        }
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class=\"quote\">\n");
        html.push_str("<div class=\"text\">");
        html.push_str(&self.text);
        html.push_str("</div>\n");
        html.push_str("<div class=\"source\">");
        html.push_str(&self.source);
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html
    }
}

pub fn quotes_content() -> String {
    let mut html = String::new();
    html.push_str("<div class=\"quotes\">\n");
    let mut quote = Quote::new("they sowed their isn't they reaped their same\n sun moon stars rain", "E.E. Cummings").to_html();
    html.push_str(quote.as_str()); 
    quote = Quote::new("The only thing I ever learned is that some people are lucky and some people aren't and not even a graduate of the Harvard Business School can say why.", "Kurt Vonnegut, The Sirens of Titan").to_html(); 
    html.push_str(quote.as_str()); 
    quote = Quote::new("I knew that nothing stranger had ever happened, that nothing stranger could ever happen. Why should I be my aunt, or me, or anyone?", "Elizabeth Bishop, In the Waiting Room").to_html(); 
    html.push_str(quote.as_str()); 
    quote = Quote::new("What similarities--\n boots, hands, the family voice\n I felt in my throat...\n 
    held us all together\n or made us all just one?\nHow-I didn't know any\n word for it-how unlikely...", "Elizabeth Bishop, In the Waiting Room").to_html();
    html.push_str(quote.as_str());
    quote = Quote::new("So it goes.", "Kurt Vonnegut, Slaughterhouse-Five").to_html(); 
    html.push_str(quote.as_str()); 
    quote = Quote::new("You realize that nature is ruthless and our existence is very fragile, temporary, and precious. But to go on with your daily affairs, you can't really think about that...which is probably why everyone takes the world for granted and why we act so thoughtlessly. It's very confusing. I suppose it will all make sense when we grow up.", "Bill Watterson, Calvin & Hobbes").to_html(); 
    html.push_str(quote.as_str());
    quote = Quote::new("We have a right to know what's going on!", "Kurt Vonnegut, The Sirens of Titan").to_html(); 
    html.push_str(quote.as_str());
    quote = Quote::new("If it runs and seems correct, you ship it. It's good enough", "a sarcastic Software Engineering professor").to_html(); 
    html.push_str(quote.as_str()); 
    quote = Quote::new("In real life, everything you do can and probably will fail.", "that same Software Engineering professor").to_html();
    html.push_str(quote.as_str()); 
    quote = Quote::new("There's who you are, and there's who you think you are, and your personal misery is defined by how big the gap is between them.", "Glee").to_html(); 
    html.push_str(quote.as_str()); 
    quote = Quote::new("The greatest hazard of all, losing one's self, can happen very quietly in the world, as though it were nothing at all.", "Soren Kierkegaard").to_html(); 
    html.push_str(quote.as_str()); 
    html.push_str("</div>\n");
    html
}
