# Double 0 project.



## What works

* simple config, that specifies which addresses are whitelisted
* proxy implementation
    * returns 403 on most things
    * filters out transaction hashes from blocks
    * allows any transactions (including new contract deployments)
* also a middle ware that takes the requests with authorization and forwards them accordingly.



## Starting system against the localhost
```shell
cargo run -- --sequencer-url http://localhost:8011  run
```

## Adding credentials
```shell
curl --request POST \                                                                                                                             
     --url http://localhost:8015 \
     --header 'accept: application/json' \
     --header 'content-type: application/json' \
     --data '
{
  "jsonrpc": "2.0",
  "method": "privateeth_addCredential",
  "id": 1,
  "params": [
    "abcd", "0x36615Cf349d7F6344891B1e7CA7C72883F5dc049", "sth"
  ]
}'
```


## Stuff to add

* persist cookies over time
* actual checking of the authorization
* simple frontend
