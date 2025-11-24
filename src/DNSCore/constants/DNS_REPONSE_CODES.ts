const DNS_RESPONSE_CODES: ReadonlyMap<number, string> = new Map<number, string>([
  [0, 'NO_ERROR'] as const,
  [1, 'FORMAT_ERROR'] as const,
  [2, 'SERVER_FAILURE'] as const,
  [3, 'NAME_ERROR'] as const,
  [4, 'NOT_IMPLEMENTED'] as const,
  [5, 'REFUSED'] as const,
]);

Object.freeze(DNS_RESPONSE_CODES);

export { DNS_RESPONSE_CODES };
