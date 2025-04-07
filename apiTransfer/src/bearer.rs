pub mod bearer {

    pub struct BearersToken {
        pub token: String,
    }

    pub struct Bearers {
        pub bearer_token : BearersToken,
    }

    impl Bearers {
        pub fn new() -> Self {
            let bar = BearersToken { token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJtYWNoaW5lX251bWJlciI6ImlzX2luX2FwcCIsImlhdCI6MTYzNzA2ODgwNywiZXhwIjoxNzM3MDcyNDA3fQ.THIO_cgdmjQFQLHvn-dK1_DNLkRKeVj3APOZLlx7ayIAjJJIT6kRBMruXAV7MvHAO44BaceVpKk2udDgh1L0Ju_GZLbASnq3kgQTCEf_AEwnquJ2voZxoHOBt_f0lZpeNRwrwribmaejrOzFUvLK6Yzf1tPlZS4VupP0qMbk8ctG7vGhW-2udr8npK-Vz4zRmR5IxAxZgGxCmf24AW3FCDYbHPK9BodV9Ge3VdrCfXbmhE0zAi6LFWK-UrXSCmTJ-524YpPi2_1ACmeKck8uxiTtV6b69eUpu6DRyeWM1UQdMliThrrUbzwzb-Gi24B6xbl0NYPUCKROXQvjp21sxw".to_string()};

            Bearers { bearer_token: bar}
        }
    }
}

pub use bearer::Bearers;