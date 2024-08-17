<template>
  <div>
    <div v-if="isAuthorized">
      Your account is authorized.
      
      <slot />

    </div>
    <template v-else>
      You have to sign a text message to tell the server that you really own this account. <br>
      This is NOT a transaction - just a text message. <br>
      <button @click="authorize()">Authorize</button>
    
    </template>

    

  </div>
</template>

<script lang="ts" setup>
import { recoverMessageAddress } from 'viem';
import { signMessage as wagmiSignMessage } from '@wagmi/core';

import axios from 'axios';

const { credential, account } = storeToRefs(useWagmi());

const isAuthorized = ref(false);



const { result, execute: checkCredential, inProgress, error} = useAsync(async () => {

  const json = JSON.stringify({ 
    jsonrpc: "2.0",
    method: "privateeth_checkCredential",
    id: 1,
    params: [
      credential.value, account.value.address
    ]
   });


  const checkCreds = await axios.post(DOUBLE_ZERO_SERVER_PREFIX, json, {
    headers: {
      'Content-Type': 'application/json'
    }
  });

  
  isAuthorized.value = checkCreds.data["result"];
    
  return {
    checkCreds
  }
});

checkCredential();

const message = "Access to 00: " + credential.value;

const {  execute: authorize } = useAsync(async () => {

  const signature =  await wagmiSignMessage({ message: message! })
  const recoveredAddress = await recoverMessageAddress({ message: message!, signature });

  const json = JSON.stringify({ 
    jsonrpc: "2.0",
    method: "privateeth_addCredential",
    id: 1,
    params: [
      credential.value, recoveredAddress, signature
    ]
   });


  const res = await axios.post(DOUBLE_ZERO_SERVER_PREFIX, json, {
    headers: {
      'Content-Type': 'application/json'
    }
  });


  isAuthorized.value = res.data["result"];

  return {
    signature,
    recoveredAddress,
    res
  }
});



</script>
