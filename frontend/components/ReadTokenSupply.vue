<template>
  <div>
    <div>
      Total Supply: {{ supply?.toString() }}
      <button @click="fetchTotalSupply">{{ inProgress ? 'fetching...' : 'refetch' }}</button>
    </div>
    <div v-if="error">Error: {{ error?.message }}</div>
  </div>
</template>

<script lang="ts" setup>
import { erc20ABI, readContract } from '@wagmi/core';

const props = defineProps<{ contractAddress: `0x${string}` }>();


const { result: supply, execute: fetchTotalSupply, inProgress, error} = useAsync(async () => {
  return await readContract({
    address: props.contractAddress,
    abi: erc20ABI,
    functionName: 'totalSupply',
  })
});

fetchTotalSupply();
</script>