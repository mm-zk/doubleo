<template>
  <div>
    <button v-if="account.isConnected" @click="disconnect">
      Disconnect from {{ account.connector?.name }}
    </button>

    <template v-else>
      Please connect your wallet first - so that we can tell the server which accounts are allowed to use this credential.<br>
      <button v-for="item in config.connectors" :key="item.id" @click="connect({ connector: item })">
        {{ item.name }}
      </button>
    </template>
  </div>
</template>

<script lang="ts" setup>
import { connect, disconnect, getConfig } from '@wagmi/core';

const config = getConfig();
const { account } = storeToRefs(useWagmi())
</script>