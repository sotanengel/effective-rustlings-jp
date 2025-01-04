use time::macros::date;
use time::Date;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Event {
    title: String,
    date: Option<Date>,
    description: Option<String>,
}

// ビルダーの実装
struct EventBuilder(Event);

impl EventBuilder {
    fn new(title: &str) -> Self {
        EventBuilder(Event {
            title: title.to_string(),
            date: None,
            description: None,
        })
    }

    // TODO: 段階的にビルダに情報をセットできるように引数と返り値の型を調整してください。
    fn date(mut self, date: Date) -> Self {
        self.0.date = Some(date);
        self
    }

    // TODO: 段階的にビルダに情報をセットできるように引数と返り値の型を調整してください。
    fn description(mut self, description: &str) -> Self {
        self.0.description = Some(description.to_string());
        self
    }

    // TODO: 1つのビルダから複数のインスタンスを作成できるように「引数の型」と「関数内部」を実装してください。
    fn build(self) -> Event {}
}

fn main() {
    let mut event_builder = EventBuilder::new("Rust.Tokyo");

    // 段階的にビルダーに情報をセットする
    event_builder.date(date!(2024 - 11 - 30));
    event_builder.description("2023年にデジタル戦略の立案・支援・人財育成を担う組織として、デジタル・イノベーション推進部を新設しました。");

    // 配列に同じビルダー情報から作ることができるインスタンスを生成する
    let event_list = vec![
        event_builder.build(),
        event_builder.build(),
        event_builder.build(),
    ];

    println!("{:?}", event_list);
}
