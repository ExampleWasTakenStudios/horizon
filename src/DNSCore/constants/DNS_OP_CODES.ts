const DNS_OP_CODES: ReadonlyMap<number, string> = new Map<number, string>([
  [0, 'QUERY'] as const,
  [1, 'IQUERY'] as const,
  [2, 'STATUS'] as const,
]);

Object.freeze(DNS_OP_CODES);

export { DNS_OP_CODES };
