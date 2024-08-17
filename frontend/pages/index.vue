<template>
  <div>
    <h1>Double-zero demo</h1>

    <p>
      Welcome to double zero demo - a privacy proxy over the ZK chains.
    </p>

    <BasicInfo />
      <hr />

    <Connect />

    <Connected>

      <hr />
      <h2>Account</h2>
      <Account />
      <br />
      <hr />

      <Authorize>
        <hr />
        <h2>Network</h2>
        <NetworkSwitcher>
          <hr />
          <h2>Balance</h2>
          <b>You can see balance of your wallet...</b> <br>
          <Balance />
          <br />

          <b>But you'll not be able to see balances of others...</b>
          
          <FindBalance />

          <br>

          <hr />

          <h2> Tokens (ERC20 - fully public)</h2>
          Address: 0x111C3E89Ce80e62EE88318C2804920D4c96f92bb
          <ReadTokenSupply :contractAddress="'0x111C3E89Ce80e62EE88318C2804920D4c96f92bb'" />
          <ReadTokenBalance :contractAddress="'0x111C3E89Ce80e62EE88318C2804920D4c96f92bb'"/>




          <hr />

          <h2> Tokens (ERC20 - private)</h2>
          Address: 0x4B5DF730c2e6b28E17013A1485E5d9BC41Efe021
          <br>
          <b>You can always fetch the total supply...</b>
          <ReadTokenSupply :contractAddress="'0x4B5DF730c2e6b28E17013A1485E5d9BC41Efe021'" />
          <ReadTokenBalance :contractAddress="'0x4B5DF730c2e6b28E17013A1485E5d9BC41Efe021'"/>




        </NetworkSwitcher>
      </Authorize>
    </Connected>
  </div>
</template>

<script lang="ts" setup>
useWagmi();
</script>
