<template>
  <div>
    <div>
      Token balance: {{ balance?.toString() }}
    </div>
    <div>
      <input v-model="address" type="text" placeholder="wallet address">
      <button @click="fetchBalance">{{ inProgress ? 'fetching...' : 'refetch' }}</button>
    </div>
    <div v-if="error">Error: {{ error?.message }}</div>
  </div>
</template>

<script lang="ts" setup>
import { erc20ABI, readContract } from '@wagmi/core';

const { account } = storeToRefs(useWagmi());
const address = ref(account.value.address);
const props = defineProps<{ contractAddress: `0x${string}` }>();


const { result: balance, execute: fetchBalance, inProgress, error} = useAsync(async () => {
  return await readContract({
    address: props.contractAddress,
    abi: erc20ABI,
    functionName: 'balanceOf',
    args: [address.value!],
  })
});

fetchBalance();
</script>
