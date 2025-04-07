pub mod config {
    pub struct Network {
        pub decimal: f32,
        pub contract_address: Option<String>,
    }

    pub struct BearerToken {
        pub token: String,
    }

    pub struct Config {
        pub sui: Network,
        pub bearer : BearerToken,
    }

    impl Config {
        pub fn new() -> Self {
            let network_sui = Network { decimal: 9.0, contract_address: None };
            let bar = BearerToken { token: "Bearer eyJ02ODgwjrOodVliThrrUbzwzb-Gi24B6xbl0NYPUCKROXQvjp21sxw".to_string()};

            Config { sui: network_sui,bearer: bar}
        }
    }
}

pub use config::Config;