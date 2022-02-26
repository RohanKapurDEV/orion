import BN from "bn.js";
import { PublicKey } from "@solana/web3.js";

// .to_le_bytes() for u64
export const bnTo8 = (bn: BN): Uint8Array => {
  return Buffer.from([...bn.toArray("le", 8)]);
};

// .to_le_bytes() for u16
export const bnTo16 = (bn: BN): Uint8Array => {
  return Buffer.from([...bn.toArray("le", 2)]);
};

// .to_le_bytes() for u8
export const bnTo1 = (bn: BN): Uint8Array => {
  return Buffer.from([...bn.toArray("le", 1)]);
};

// sleeper function
export const delay = (milliseconds) => {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
};
