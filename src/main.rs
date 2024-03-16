use reqwest::blocking;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
struct Post {
    title: String,
    link: String,
    pub_date: String,
}

fn fetch_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let mut posts = Vec::new();
    let url = "https://yossy.dev";
    let resp = blocking::get(url)?.text()?;
    let document = Html::parse_document(&resp);
    let post_selector = Selector::parse("li.text-lg").unwrap();
    for post in document.select(&post_selector) {
        let title = post
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let link_suffix = post
            .select(&Selector::parse("a").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        let link = format!("https://yossy.dev{}", link_suffix);
        let pub_date = post.text().collect::<Vec<_>>()[0].trim().to_string();
        posts.push(Post {
            title,
            link,
            pub_date,
        });
    }

    Ok(posts)
}

fn generate_rss(posts: Vec<Post>) -> String {
    let mut rss = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n<channel>\n<title>yossy.dev Blog</title>\n<link>https://yossy.dev</link>\n<description>yossydev's personal blog</description>\n");

    for post in posts {
        let item = format!(
            "<item>\n<title>{}</title>\n<link>{}</link>\n<guid>{}</guid>\n<pubDate>{}</pubDate>\n</item>\n",
            post.title,
            post.link,
            post.link,
            post.pub_date
        );
        rss.push_str(&item);
    }

    rss.push_str("</channel>\n</rss>");
    rss
}

fn main() -> Result<(), Box<dyn Error>> {
    let posts = fetch_posts()?;
    let rss_feed = generate_rss(posts);
    println!("{}", rss_feed);
    Ok(())
}
