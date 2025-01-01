
# The Ethereum Fetcher - REST Server

REST API server that returns information for certain Ethereum transactions identified by their transaction hashes.
## How to run the project

Clone the project

```bash
  git clone https://github.com/nikolaicholakov25/eth_fetcher
```

Go to the project directory

```bash
  cd eth_fetcher
```

#### To run this project, you will need to add the following environment variables to your .env file

`API_PORT=3000`

`ETH_NODE_URL=` (the service was developed and tested with [infura.io](https://www.infura.io/) url)

`DB_CONNECTION_URL=` (PostgreSQL connection)

`JWT_SECRET=`


Start the local server

```bash
  cargo run
```

**or**

Create and run a docker image

```bash
  docker build -t limeapi .
```
```bash
  docker run -p 3000:3000 limeapi
```

#### !Note that the docker port (exposed:docker) must be the same as the `API_PORT` env variable


## API Reference GET ENDPOINTS

#### Get eth transactions by their hashes

```http
  GET /lime/eth?transactionHashes=0x...,0x...
```

| Query Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `transactionHashes` | `string[]` | The hashes of the transactions you want to fetch  |

#### Get eth transactions by **rlp encoded** transaction hashes list

```http
  GET /lime/eth/:rlphex
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `rlphex`      | `string` | The **rlp encoded** list of transaction hashes you want to fetch |

#### Each transaction, once fetched will be saved in a **PostgreSQL Database**, this endpoint returns all saved transactions

```http
  GET /lime/all
```

#### Calling endpoints /lime/eth?transactionHashes and /lime/eth/:rlphex, when authenticated will save your searches in the database. Calling this endpoint will return all searched transactions

```http
  GET /lime/my
```

| Request Header | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `AUTH_TOKEN` **required**      | `string` | The jwt token returned from **POST /lime/authenticate** |


## API Reference POST ENDPOINTS

#### Authenticate and receive a jwt auth token for subsequent requests

```http
  POST /lime/authenticate
```

| Request Body | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `username`      | `string` | The user's username |
| `password`  | `string` | The user's password |

### Available username/password combinations
- alice/alice
- bob/bob
- carol/carol
- dave/dave

###

| Request Header | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `AUTH_TOKEN` **optional**      | `string` | The jwt token returned from **POST /lime/authenticate** |

## Examples of sepolia transactions

#### Fetch transaction data from transaction hashes
Request
```http
  GET /lime/eth?transactionHashes=0xbdb191d7ee7c25cc144b4ba35ea06cc912762495a66ad0336a7cabc5ab31c36f,0x81d89a76c55b3a7460bcb189ae930f25e8be566e44f22036e32e2f5765f82bab
```
Response
```json
{
  "transactions": [
    {
      "transactionHash": "0xbdb191d7ee7c25cc144b4ba35ea06cc912762495a66ad0336a7cabc5ab31c36f",
      "transactionStatus": 1,
      "blockHash": "0xfc86f7f2462c4f5e1ed09a6af4e458950063b91bc926d65d8f6022b2b383d89e",
      "blockNumber": 7392640,
      "from": "0x5520a8a1723Fdc8a8e64Da1f348CC1991C13C1C3",
      "to": "0xea58fcA6849d79EAd1f26608855c2D6407d54Ce2",
      "contractAddress": null,
      "logsCount": 5,
      "input": "0xe11013dd0000000000000000000000005520a8a1723fdc8a8e64da1f348cc1991c13c1c30000000000000000000000000000000000000000000000000000000000030d400000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000b7375706572627269646765000000000000000000000000000000000000000000",
      "value": "10000000000000000"
    },
    {
      "transactionHash": "0x81d89a76c55b3a7460bcb189ae930f25e8be566e44f22036e32e2f5765f82bab",
      "transactionStatus": 1,
      "blockHash": "0x56bf81f121cb2e18fb62fd014f90e890b077e6935e4d3c52d50c9aef6489485c",
      "blockNumber": 7394027,
      "from": "0xDD286012b76112bA00A872d47659b00B0531c0A0",
      "to": "0x5f5a404A5edabcDD80DB05E8e54A78c9EBF000C2",
      "contractAddress": null,
      "logsCount": 5,
      "input": "0xe11013dd000000000000000000000000dd286012b76112ba00a872d47659b00b0531c0a00000000000000000000000000000000000000000000000000000000000030d400000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000b7375706572627269646765000000000000000000000000000000000000000000",
      "value": "10000000000000000"
    }
  ]
}
```
#### Fetch transaction data from rlp encoded list

Request
```http
  GET /lime/eth/f863a08e5484577d7f6bc0dd7d6a7016a55e3e33a43ece50c4c11aad074b3d728a8d35a07addeb71d33c4824e31b30d92894e0e1d2e0c0a13d8e1020aaad80d5b3ee32eca0031b20239d55bee927ab7bc0510748628438a6d08dccffbb0da61f3b72bc71ed
```
Response
```json
{
  "transactions": [
    {
      "transactionHash": "0x8e5484577d7f6bc0dd7d6a7016a55e3e33a43ece50c4c11aad074b3d728a8d35",
      "transactionStatus": 1,
      "blockHash": "0x52089bbb250319edde228f853371923688956fdef5a23bff33c2ef4825ba3812",
      "blockNumber": 7378870,
      "from": "0xf2Ff1b18879F14B575a9C7866AA829AF9456dE6D",
      "to": "0xea58fcA6849d79EAd1f26608855c2D6407d54Ce2",
      "contractAddress": null,
      "logsCount": 5,
      "input": "0xe11013dd000000000000000000000000f2ff1b18879f14b575a9c7866aa829af9456de6d0000000000000000000000000000000000000000000000000000000000030d400000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000b7375706572627269646765000000000000000000000000000000000000000000",
      "value": "70000000000000008"
    },
    {
      "transactionHash": "0x7addeb71d33c4824e31b30d92894e0e1d2e0c0a13d8e1020aaad80d5b3ee32ec",
      "transactionStatus": 1,
      "blockHash": "0x52089bbb250319edde228f853371923688956fdef5a23bff33c2ef4825ba3812",
      "blockNumber": 7378870,
      "from": "0xBD69e2001001FDB84d6E797eB452a7eEE730F152",
      "to": "0x8E24480cfe2cD9683EDaaB062F3877b47A9beF4c",
      "contractAddress": null,
      "logsCount": 1,
      "input": "0x9aaab648c3c393f6ae6343aa266810bd344a68b5a4b3591fec55a840ebe4fe62744e31550000000000000000000000000000000000000000000000000000000000d4d438cd46c9698af1c69201ea4edbdfb7f2e56f55b68f18704fa2750d0b290a1f5a4100000000000000000000000000000000000000000000000000000000007097ab",
      "value": "0"
    },
    {
      "transactionHash": "0x031b20239d55bee927ab7bc0510748628438a6d08dccffbb0da61f3b72bc71ed",
      "transactionStatus": 1,
      "blockHash": "0x52089bbb250319edde228f853371923688956fdef5a23bff33c2ef4825ba3812",
      "blockNumber": 7378870,
      "from": "0xd53Eb5203e367BbDD4f72338938224881Fc501Ab",
      "to": "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
      "contractAddress": null,
      "logsCount": 2,
      "input": "0x1fad948c0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000d53eb5203e367bbdd4f72338938224881fc501ab00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000f867c3bdf7d4f3fcc36ca1c7cb7d5b7ee3aa7fac000000000000000000000000000000000000000000000000000000000000795800000000000000000000000000000000000000000000000000000000000001600000000000000000000000000000000000000000000000000000000000000180000000000000000000000000000000000000000000000000000000000000238c0000000000000000000000000000000000000000000000000000000000011eb5000000000000000000000000000000000000000000000000000000000000b2bc0000000000000000000000000000000000000000000000000000000b8c208435000000000000000000000000000000000000000000000000000000000eb70041000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000002a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004434fcd5be00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000075c03aac639bb21233e0139381970328db8bceeb670000000000000000677141040000000000000000000000000000000000000000d5a610202a2933f6932a1cb5472517e3d8bc8ea46a9bf6fb7be247cdc1301afe2fec2759af28e955f159fcab588b2db9c6c6f1fe6f57fc550270549c27cfe5d81c00000000000000000000000000000000000000000000000000000000000000000000000000000000000045000000004c23105d9a50be8b8930d7bfe424ab6e9f985aceb480773a66d83d5f34eebd0539b273fe2eb3e36ba2ddb985bc4761ec2a2cae6a5cb825834bced57e264497901c000000000000000000000000000000000000000000000000000000",
      "value": "0"
    }
  ]
}
```

#### Authenticate

Request
```http
  POST /lime/authenticate
```
Body
```json
{
  "username": "bob",
  "password": "bob"
}
```
Response
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjoiYm9iIiwiZXhwIjoxNzM1NzgwOTkxfQ.i3NJyHt5j_mo40NtBddNVwcJ2dMmNo7o1-3sL2Ud6xU"
}
```
## How to test

```rust
cargo test
```
