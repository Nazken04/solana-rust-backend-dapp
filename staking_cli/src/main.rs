use structopt::StructOpt;
use serde::{Deserialize, Serialize};

#[derive(Debug, StructOpt)]
#[structopt(name = "staking-cli", about = "A CLI to interact with the staking dApp.")]
enum Command {
    Stake {
        #[structopt(short, long)]
        user: String,
        #[structopt(short, long)]
        amount: u64,
    },
    Unstake {
        #[structopt(short, long)]
        user: String,
        #[structopt(short, long)]
        amount: u64,
    },
    Balance {
        #[structopt(short, long)]
        user: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct StakeRequest {
    user_public_key: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnstakeRequest {
    user_public_key: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BalanceResponse {
    balance: u64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let command = Command::from_args();
    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8080";

    match command {
        Command::Stake { user, amount } => {
            let res = client
                .post(format!("{}/stake", base_url))
                .json(&StakeRequest { user_public_key: user, amount })
                .send()
                .await?
                .text()
                .await?;
            println!("{}", res);
        }
        Command::Unstake { user, amount } => {
            let res = client
                .post(format!("{}/unstake", base_url))
                .json(&UnstakeRequest { user_public_key: user, amount })
                .send()
                .await?
                .text()
                .await?;
            println!("{}", res);
        }
        Command::Balance { user } => {
            let res: BalanceResponse = client
                .get(format!("{}/balance/{}", base_url, user))
                .send()
                .await?
                .json()
                .await?;
            println!("Balance: {}", res.balance);
        }
    }

    Ok(())
}
