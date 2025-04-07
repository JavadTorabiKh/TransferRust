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
            let bar = BearerToken { token: "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJtYWNoaW5lX251bWJlciI6ImlzX2luX2FwcCIsImlhdCI6MTYzNzA2ODgwNywiZXhwIjoxNzM3MDcyNDA3fQ.THIO_cgdmjQFQLHvn-dK1_DNLkRKeVj3APOZLlx7ayIAjJJIT6kRBMruXAV7MvHAO44BaceVpKk2udDgh1L0Ju_GZLbASnq3kgQTCEf_AEwnquJ2voZxoHOBt_f0lZpeNRwrwribmaejrOzFUvLK6Yzf1tPlZS4VupP0qMbk8ctG7vGhW-2udr8npK-Vz4zRmR5IxAxZgGxCmf24AW3FCDYbHPK9BodV9Ge3VdrCfXbmhE0zAi6LFWK-UrXSCmTJ-524YpPi2_1ACmeKck8uxiTtV6b69eUpu6DRyeWM1UQdMliThrrUbzwzb-Gi24B6xbl0NYPUCKROXQvjp21sxw".to_string()};

            Config { sui: network_sui,bearer: bar}
        }
    }
}

pub use config::Config;