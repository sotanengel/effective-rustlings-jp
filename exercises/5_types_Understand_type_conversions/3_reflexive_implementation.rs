// ドメイン名を表す構造体
#[derive(Debug)]
pub struct DomainName(pub String);

// StringからDomainNameへの変換
impl From<String> for DomainName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

// &str からも直接変換できるようにする
impl From<&str> for DomainName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

// ドメインが example.com のサブドメインかを判定する関数
// TODO: 反射実装を使って、この関数にトレイト制約をInto<DomainName>に課すことで
//　　　 「String」「&str」「DomainName」を引数に入れたとしても対応できるようにしてください。
pub fn is_example_subdomain<T>(domain: T) -> bool {
    let domain = domain.into();
    domain.0.ends_with(".example.com")
}

fn main() {
    let domain1 = DomainName("sub.example.com".to_string());
    let domain2 = "other.com".to_string();
    let domain3 = "api.example.com";

    // DomainName構造体を直接渡す
    println!("sub.example.com? {}", is_example_subdomain(domain1));

    // Stringを渡す (Intoで自動変換)
    println!("other.com? {}", is_example_subdomain(domain2));

    // &strを渡す (Intoで自動変換)
    println!("api.example.com? {}", is_example_subdomain(domain3));
}
