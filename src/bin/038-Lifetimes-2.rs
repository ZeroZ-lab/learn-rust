struct Content<'a,'b> {
    content: &'a str,
    author: &'b str,
}

impl<'a,'b> Content<'a,'b> {
    fn new(content: &'a str, author: &'b str) -> Self {
        Self { content, author }
    }
}


fn main() {
    let content = Content::new("内容", "作者");
    println!("内容：{}，作者：{}", content.content, content.author);
}
