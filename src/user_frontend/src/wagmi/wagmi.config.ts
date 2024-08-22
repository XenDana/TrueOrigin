import { createConfig, http } from "wagmi";

import { mainnet } from "wagmi/chains";
import { walletConnect } from "wagmi/connectors";

// Replace with your own WalletConnect project ID
// Register for free at https://walletconnect.com/
const WALLETCONNECT_PROJECT_ID = "e186ab3a3109cae9e842c88a455e4e44";

export const wagmiConfig = createConfig({
  chains: [mainnet],
  connectors: [walletConnect({ projectId: WALLETCONNECT_PROJECT_ID })],
  transports: {
    [mainnet.id]: http(),
  },
});
