use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use scraper::{Html, Selector};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Post {
    title: String,
    link: String,
    pub_date: String,
}

async fn fetch_posts(url: &str) -> Result<Vec<Post>> {
    let resp = ureq::get(url).call()?.into_string()?;
    let document = Html::parse_document(&resp);
    let post_selector = Selector::parse("li.text-lg").unwrap();

    let posts: Vec<Post> = document
        .select(&post_selector)
        .map(|post| {
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

            let mut foo;
            if link_suffix.contains("https://") {
                foo = url.replace("https://yossy.dev/all", "");
            } else {
                foo = url.replace("/all", "");
            };

            let link = format!("{}{}", foo, link_suffix);
            println!("{}", link);
            let pub_date = post.text().collect::<Vec<_>>()[0].trim().to_string();
            let formatted_date = format_date(&pub_date);

            Post {
                title,
                link,
                pub_date: formatted_date,
            }
        })
        .collect();

    Ok(posts)
}

fn format_date(date_str: &str) -> String {
    if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y/%m/%d") {
        let offset = FixedOffset::east_opt(9 * 3600);
        match offset {
            Some(i) => {
                let datetime = DateTime::<FixedOffset>::from_local(date, i);
                datetime.format("%a, %d %b %Y %T %Z").to_string()
            }
            // println!を使用するとエラーにになる ref: https://github.com/rust-lang/rust/issues/24157#issuecomment-303826608
            None => panic!("It's not one!"),
        }
    } else {
        date_str.to_string()
    }
}

fn generate_rss(posts: Vec<Post>) -> String {
    let header = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<rss version=\"2.0\">\n<channel>\n<title>yossy.dev Blog</title>\n<link>https://yossy.dev</link>\n<description>yossydev's personal blog</description>\n";
    let footer = "</channel>\n</rss>";
    let items: String = posts.into_iter().map(|post| {
        format!(
            "<item>\n<title>{}</title>\n<link>{}</link>\n<guid>{}</guid>\n<pubDate>{}</pubDate>\n</item>\n",
            post.title, post.link, post.link, post.pub_date
        )
    }).collect();
    format!("{}{}{}", header, items, footer)
}

async fn rss_feed() -> impl Responder {
    let posts = fetch_posts("https://yossy.dev/all")
        .await
        .unwrap_or_else(|_| vec![]);
    let rss_feed = generate_rss(posts);
    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(rss_feed)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/rss", web::get().to(rss_feed)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
