<div style="display:grid; justify-items:center;">
    <img src="misc/areoxide-logo.png" width="150" height="150" />
    <h1>Areoxide</h1>
    <div style="margin-bottom: 8px;">
        <img src="https://img.shields.io/badge/Areon%20Network-00CCFd" />
        <img src="https://img.shields.io/badge/Open%20Source-green"/>
        <img src="https://img.shields.io/crates/d/areoxide" />
        <img src="https://img.shields.io/github/languages/top/alperencantez/areoxide" />
        <img src="https://img.shields.io/badge/WASM-development-purple" />
    </div>
</div>

Areoxide is a lightweight and efficient library designed to wrap RPC methods for the Areon Network. It simplifies the process of interacting with Areon Network's RPC endpoints, enabling developers to build robust applications with ease.

---

## Features

- **Simplified API**: Intuitive functions to interact with Areon Network RPC endpoints.
- **Lightweight**: Minimal dependencies for easy integration.
- **Flexible**: Supports various use cases, including querying network data and more.
- **Extensible**: Easily extendable to include additional Areon RPC methods.

<!-- ## Installation

Add Areoxide to your project using your preferred package manager.

For example, with Cargo:

```bash
cargo add areoxide
``` -->

---

## Getting Started

### Importing the Library

To use Areoxide, first, include it in your Rust project:

```rust
use areoxide::prelude::{rpc::RpcClient};
```

### Initialization

Create an instance of the `AreonClient` by providing the RPC endpoint URL:

```rust
const RPC_URL: &str = "https://mainnet-rpc.areon.network";
let client: RpcClient = RpcClient::init(RPC_URL);
```

### Example Usage

> You will probably need `areoxide::prelude::util` and `areoxide::prelude::constants` modules in order to make conversions from hexadecimal to decimal and vice versa. You can import it with:

```rust
use areoxide::prelude::{util, constants};
```

#### Onchain balance control

```rust
   match client.get_balance("0x".to_string()).await {
        Ok(balance) => {
            println!("Balance: {:#?}", util::hexadecimal_str_to_decimal_str(balance).unwrap())
        }
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
```

#### Get contract code

```rust
    match client.get_code("0x".to_string(), constants::BLOCK_STATE_LATEST.to_string()).await {
        Ok(code) => println!("Address code: {:#?}", code),
        Err(e) => eprintln!("Error fetching code: {}", e),
    }
```

#### Transaction Receipt

```rust
 match client.get_transaction_receipt("0x".to_string()).await {
        Ok(receipt) => println!("TX receipt: {:#?}", receipt),
        Err(e) => eprintln!("Error fetching tx receipt: {}", e),
}
```

---

<!--
## Documentation

Comprehensive documentation is available [here](https://areoxide-docs.example.com).

- **Getting Started**
- **API Reference**
- **Examples**
- **Contributing**

--- -->

## Contributing

We welcome contributions to improve Areoxide! Feel free to submit issues, feature requests, or pull requests to the GitHub repository.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m 'Add some feature'`)
4. Push to the branch (`git push origin feature-name`)
5. Open a pull request

## License

Areoxide is licensed under the [MIT License](./LICENSE).
