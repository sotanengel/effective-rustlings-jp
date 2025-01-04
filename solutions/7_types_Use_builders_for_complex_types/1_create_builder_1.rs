use time::macros::date;
use time::Date;

#[derive(Debug)]
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

    fn date(mut self, date: Date) -> Self {
        self.0.date = Some(date);
        self
    }

    fn description(mut self, description: &str) -> Self {
        self.0.description = Some(description.to_string());
        self
    }

    // インスタンスを生成
    fn build(self) -> Event {
        self.0
    }
}

fn main() {
    let event = EventBuilder::new("Rust.Tokyo")
        .date(date!(2024 - 11 - 30)) // ここに日付をセット
        .description("2023年にデジタル戦略の立案・支援・人財育成を担う組織として、デジタル・イノベーション推進部を新設しました。")
        .build();

    println!("{:?}", event);
}
