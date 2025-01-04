use time::macros::date;
use time::Date;

#[derive(Debug)]
#[allow(dead_code)]
struct Event {
    title: String,
    date: Option<Date>,
    description: Option<String>,
}

// TODO: 「Event構造体そのものをタプル構造体で保持する」EventBuilderという名前のビルダを作成してください。

impl EventBuilder {
    // TODO: titleを入力するとEventBuilderを返すnew関数を実装してください。
    fn new(title: &str) -> Self {}

    // TODO: dateを入力するとEventBuilderに「日付情報」を追加するメソッドを実装してください。
    fn date(mut self, date: Date) -> Self {}

    // TODO: descriptionを入力するとEventBuilderに「イベント詳細情報」を追加するメソッドを実装してください。
    fn description(mut self, description: &str) -> Self {}

    // TODO: Eventインスタンスを作成するbuildメソッドを実装してください。
    fn build(self) -> Event {}
}

fn main() {
    let event = EventBuilder::new("Rust.Tokyo")
        .date(date!(2024 - 11 - 30)) // ここに日付をセット
        .description("2023年にデジタル戦略の立案・支援・人財育成を担う組織として、デジタル・イノベーション推進部を新設しました。")
        .build();

    println!("{:?}", event);
}
