use cursive::{
    view::{Nameable, Scrollable},
    views::{Dialog, EditView, ListView, TextView},
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
            .title("search")
            .padding_lrtb(1, 1, 1, 0)
            .content(EditView::new().with_name("search"))
            .button("Ok", move |s| {
                // s.pop_layer();
                rt.block_on(perform_search(s));
            }),
    );

    // Starts the event loop.
    siv.run();
}

async fn perform_search(s: &mut Cursive) {
    let mut choose = ListView::new();
    let search = s
        .call_on_name("search", |view: &mut EditView| view.get_content())
        .unwrap();
    let result = data::search(&search).await;
    for provider in result {
        for resource in provider {
            choose.add_child(
                &resource.name,
                TextView::new(
                        " ver. ".to_string()
                        + &resource.version
                        + " by "
                        + &resource.author,
                ),
            );
        }
    }
    //your tui been messed with bozo
    s.pop_layer();
    s.add_layer(choose.scrollable());
}
