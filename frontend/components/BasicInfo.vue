<template>
  <div>
    Your credential will be: {{ credential }} <br>
    The RPC URL to access the chain will be: <b>{{ DOUBLE_ZERO_SERVER_PREFIX + credential }}</b><br>
    <br>
    Please keep this information secret - anyone who knows the credential will be able to "see" your balances - <br> but will NOT be able to do any modifications.

    <br>
    <br>
  </div>
</template>

<script lang="ts" setup>
import { connect, disconnect, getConfig } from '@wagmi/core';

const config = getConfig();
const { credential } = storeToRefs(useWagmi())
</script>