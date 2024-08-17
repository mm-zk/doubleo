import { type Chain, zkSync, zkSyncSepoliaTestnet } from '@wagmi/core/chains'
import { getAccount, getNetwork, watchAccount, watchNetwork, configureChains, createConfig } from '@wagmi/core';
import { InjectedConnector } from '@wagmi/core/connectors/injected';
import { MetaMaskConnector } from '@wagmi/core/connectors/metaMask';
import { CoinbaseWalletConnector } from '@wagmi/core/connectors/coinbaseWallet';
import { publicProvider } from '@wagmi/core/providers/public';


export const DOUBLE_ZERO_SERVER_PREFIX = 'http://localhost:8015/'
export const DOUBLE_ZERO_CHAIN_ID = 299

const COOKIE_CREDENTIAL = "double-zero-credential"

const generateDZeroNetwork = (creds: String) => {
  return {
    id: DOUBLE_ZERO_CHAIN_ID,
    name: "Double zero",
    network: "zksync-double-zero",
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    rpcUrls: {
      default: {
        http: [DOUBLE_ZERO_SERVER_PREFIX + creds],
      },
      public: {
        http: [DOUBLE_ZERO_SERVER_PREFIX + creds],
      },
    },
    blockExplorers: {
      default: {
        name: 'Local Explorer',
        url: 'http://localhost:3010',
      },
    },
    testnet: true
  }

};




export const defaultChain = import.meta.env.MODE === "development" ? zkSyncSepoliaTestnet : zkSync;


function generateRandomString(length = 16) {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  const charactersLength = characters.length;
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}


export const useWagmi = defineStore("wagmi", () => {
  const cookie = useCookie(COOKIE_CREDENTIAL);
  cookie.value = cookie.value || generateRandomString(8);

  const credential = ref(cookie.value);

  console.log("Credential is " + credential);

  const chains: Chain[] = [
    zkSyncSepoliaTestnet,
    generateDZeroNetwork(cookie.value!)

  ];

  const { publicClient, webSocketPublicClient } = configureChains(
    chains,
    [
      publicProvider(),
    ],
  )

  const wagmiConfig = createConfig({
    autoConnect: true,
    connectors: [
      new MetaMaskConnector({ chains }),
      new CoinbaseWalletConnector({
        chains,
        options: {
          appName: 'wagmi',
        },
      }),
      new InjectedConnector({
        chains,
        options: {
          name: 'Injected',
          shimDisconnect: true,
        },
      }),
    ],
    publicClient,
    webSocketPublicClient,
  })

  const account = ref(getAccount());
  const network = ref(getNetwork());
  watchAccount((updatedAccount) => {
    account.value = updatedAccount;
  });
  watchNetwork((updatedNetwork) => {
    network.value = updatedNetwork;
  });

  return {
    account,
    network,
    credential,
  }
});