use lol_html::{element, html_content::ContentType, HtmlRewriter, Settings};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_name = "__lol$config.encodeUrl")]
    fn encodeUrl(url: String) -> String;

    #[wasm_bindgen(js_name = "__lol$config.prefix")]
    static PREFIX: String;
}

#[wasm_bindgen]
pub fn rewrite_html(html: String) -> String {
    let mut output = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![
                element!("[href]", |el| {
                    let href = el
                        .get_attribute("href")
                        .unwrap();

                    el.set_attribute("href", &format!("{}{}", &PREFIX.to_string(), &encodeUrl(href.to_string()))).unwrap();

                    Ok(())
                }),
                element!("[src]", |el| {
                    let src = el
                        .get_attribute("src")
                        .unwrap();

                    el.set_attribute("src", &format!("{}{}", &PREFIX.to_string(), &encodeUrl(src.to_string()))).unwrap();

                    Ok(())
                }),
                element!("[srcset]", |el| {
                    let srcset = el
                        .get_attribute("srcset")
                        .unwrap();

                    el.set_attribute("srcset", &format!("{}{}", &PREFIX.to_string(), &encodeUrl(srcset.to_string()))).unwrap();

                    Ok(())
                }),
                element!("script", |el| {
                    el.set_inner_content("no js rewriting yet so i stole your js haha", ContentType::Text);

                    Ok(())
                }),
                element!("style", |el| {
                    el.set_inner_content("no css rewriting yet so i stole your styles haha", ContentType::Text);

                    Ok(())
                })
            ],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c)
    );

    rewriter.write(html.as_bytes()).unwrap();

    rewriter.end().unwrap();
    return format!("{}", String::from_utf8(output).unwrap());
}

