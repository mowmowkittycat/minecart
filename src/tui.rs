use bytes::buf::Buf;

use cursive::{
    view::Nameable,
    views::{Dialog, EditView, SelectView, TextView},
    Cursive,
};

mod actions;
mod data;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::new()
            .title("Search")
            .padding_lrtb(1, 1, 1, 0)
            .content(EditView::new().with_name("search"))
            .button("Search", move |s| {
                rt.block_on(processor(s));
            }),
    );

    siv.run();
}

async fn processor(siv: &mut Cursive) {
    let search = siv
        .call_on_name("search", |view: &mut EditView| view.get_content())
        .unwrap();

    let mut choose = SelectView::<String>::new().on_submit(|s: &mut Cursive, url: &String| {
        let url: String = url.clone();
        s.add_layer(
            Dialog::around(TextView::new(&url))
                .button("download", move |s| {
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(download(&url, "/home/akshat/plugin.jar".to_string()))
                        .err()
                        .unwrap();
                    s.quit();
                })
                .button("close", |s| {
                    s.pop_layer();
                }),
        );
    });

    let result = data::search(&search).await;
    for provider in result {
        for resource in provider {
            choose.add_item(
                resource.name + " ver. " + &resource.version + " by " + &resource.author,
                resource.file.url,
            );
        }
    }

    //your tui been messed with bozo
    siv.add_layer(Dialog::around(choose).button("quit", |s| s.quit()));
}

async fn download(
    url: &String,
    file_name: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(url).await?;
    let mut reader = response.bytes().await?.reader();
    let mut file = std::fs::File::create(file_name)?;
    std::io::copy(&mut reader, &mut file)?;
    Ok(())
}
