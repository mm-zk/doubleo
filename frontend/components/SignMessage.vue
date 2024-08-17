<template>
  <div>
    <div>Cookie cred: {{ cookie }} </div>
    <form @submit.prevent="signMessage">
      <button type="submit">Sign Message</button>
    </form>

    <div v-if="result && !inProgress">
      <div>
        <div>Signature: {{ result.signature }}</div>
        <div>Recovered address: {{ result.recoveredAddress }}</div>
        <div>Cookie result {{result.res}} </div>

        Switch to:
        <button @click="switchNetwork({chainId: 299})">
        Double o
      </button>
      </div>
    </div>

    <div v-if="error">Error: {{ error?.message }}</div>
  </div>
</template>

<script lang="ts" setup>
import axios from 'axios';
import { switchNetwork as wagmiSwitchNetwork } from '@wagmi/core';


import { recoverMessageAddress } from 'viem';
import { signMessage as wagmiSignMessage } from '@wagmi/core';


function generateRandomString(length = 16) {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  const charactersLength = characters.length;
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

const cookie = useCookie("double-zero-credential");
cookie.value = cookie.value || generateRandomString(8);

const message = "Access to 00: " + cookie.value;

const { result, execute: signMessage, inProgress, error} = useAsync(async () => {

  const signature =  await wagmiSignMessage({ message: message! })
  const recoveredAddress = await recoverMessageAddress({ message: message!, signature });

  const json = JSON.stringify({ 
    jsonrpc: "2.0",
    method: "privateeth_addCredential",
    id: 1,
    params: [
      cookie.value, recoveredAddress, signature
    ]
   });


  const res = await axios.post(DOUBLE_ZERO_SERVER_PREFIX, json, {
    headers: {
      'Content-Type': 'application/json'
    }
  });

  return {
    signature,
    recoveredAddress,
    res

  }
});

const { execute: switchNetwork} = useAsync(wagmiSwitchNetwork);

</script>