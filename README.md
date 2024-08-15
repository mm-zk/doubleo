# Double 0 project.



## What works

* simple config, that specifies which addresses are whitelisted
* proxy implementation
    * returns 403 on most things
    * filters out transaction hashes from blocks
    * allows any transactions (including new contract deployments)
* also a middle ware that takes the requests with authorization and forwards them accordingly.



## Stuff to add

* support for calls that pass your user id.
* support for 'cookies' - plus a way to persist them over time
* actual checking of the authorization


