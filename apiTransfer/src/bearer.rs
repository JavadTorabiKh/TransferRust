pub mod bearer {

    pub struct BearersToken {
        pub token: String,
    }

    pub struct Bearers {
        pub bearer_token : BearersToken,
    }

    impl Bearers {
        pub fn new() -> Self {
            let bar = BearersToken { token: "eyJwiZXhpUrXSCmTJ-524YpUbzwzb-Gi24B6xbl0NYPUCKROXQvjp21sxw".to_string()};

            Bearers { bearer_token: bar}
        }
    }
}

pub use bearer::Bearers;