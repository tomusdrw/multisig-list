extern crate web3;

use std::thread;
use std::sync::Arc;
use std::time::Duration;

use web3::api::Namespace;
use web3::contract::Contract;
use web3::futures::Future;
use web3::futures::future::{self, Either};
use web3::types::{Address, U256};

fn main() {
    let (eloop, transport) = web3::transports::Ipc::new("/home/tomusdrw/.local/share/io.parity.ethereum/jsonrpc.ipc").unwrap();
    let transport = Arc::new(transport);
    let web3 = web3::Web3::new(transport.clone());

    let accounts = read_from_file();

    for account in accounts {
        let call = web3.eth().code(account.clone(), None);
        let call2 = web3.eth().balance(account.clone(), None);

        let transport = transport.clone();
        eloop.remote().spawn(move |_| call.and_then(move |code| {
            if code.0.is_empty() {
                Either::A(future::done(Ok(()) as Result<_, web3::Error>))
            } else {
                let contract = Contract::from_json(
                    web3::api::Eth::new(transport),
                    account.clone(),
                    include_bytes!("../wallet.abi.json"),
                    ).unwrap();
                let owners = get_owners(contract, 0);

                Either::B(call2.and_then(move |balance| {
                    owners.map(move |owners| {
                        println!("{:?}: {}", account, as_eth(&balance));
                        for owner in owners {
                            println!("..{:?}", owner);
                        }
                    }).map_err(|_e| unreachable!())
                }))
            }
        }).map_err(|e| println!("Error: {:?}", e)));
    }

    thread::sleep(Duration::from_secs(3600));
}

fn get_owners<T: web3::Transport + 'static>(contract: Contract<T>, i: u64) -> Box<Future<Item=Vec<Address>, Error=()> + Send> where
    T: Send + 'static,
    T::Out: Send + 'static,
{
    Box::new(
        contract.query("getOwner", (i, ), None, Default::default(), None).then(move |owner: Result<Address, _>| {
            match owner {
                Ok(owner) if owner == Address::default() => {
                    Either::A(future::done(Ok(vec![])))
                },
                Err(err) => {
                    println!("..{}", err);
                    Either::A(future::done(Ok(vec![Default::default()])))
                },
                Ok(owner) => {
                    Either::B(get_owners(contract, i + 1).map(move |mut owners| {
                        owners.push(owner);
                        owners
                    }))
                },
            }
        })
    )
}

fn read_from_file() -> Vec<Address> {
    use std::{fs, io};
    use std::io::BufRead;
    let file = fs::File::open("./wallets2").unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().filter_map(|x| x.unwrap().parse().ok()).collect()
}

fn as_dec(val: &U256) -> f64 {
  let len = val.0.len();
  let mut v: f64 = 0.0;
  for i in 0..len {
    v += val.0[i] as f64;
    if i != len - 1 {
      v *= 256.0;
    }
  }
  v
}

fn as_eth(val: &U256) -> String {
    let v = as_dec(val);
    format!("{:.5}Ξ", v / 1e18)
}
