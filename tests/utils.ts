import BN from "bn.js";

export const bnTo8 = (bn: BN): Uint8Array => {
  return Buffer.from([...bn.toArray("le", 8)]);
};

export const bnTo1 = (bn: BN): Uint8Array => {
  return Buffer.from([...bn.toArray("le", 1)]);
};
