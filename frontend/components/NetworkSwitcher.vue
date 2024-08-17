<template>
  <div>
    <div>
      Connected to {{network.chain?.name ?? network.chain?.id}}
      <span v-if="network.chain?.unsupported">(unsupported)</span>
    </div>
    <br />
    <div v-if="network.chain?.id != DOUBLE_ZERO_CHAIN_ID">
      Switch to:
      <button @click="switchNetwork({chainId: DOUBLE_ZERO_CHAIN_ID})">
        Double zero 
      </button>
      (notice - if you used this chain before, make sure that the RPC has the current credentials)
    </div>
    <div v-if="error">Error: {{ error?.message }}</div>
  </div>
  <div v-if="network.chain?.id == DOUBLE_ZERO_CHAIN_ID">
    <slot />
  </div>

</template>

<script lang="ts" setup>
import { switchNetwork as wagmiSwitchNetwork } from '@wagmi/core';

const {  network } = storeToRefs(useWagmi());

const { execute: switchNetwork, inProgress, error} = useAsync(wagmiSwitchNetwork);
</script>