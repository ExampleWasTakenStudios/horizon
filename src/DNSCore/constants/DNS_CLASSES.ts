const DNS_CLASSES = new Map<number, string>([
  [1, 'IN'],
  [3, 'CH'],
  [4, 'HS'],
]);

const DNS_QCLASSES = new Map<number, string>([...DNS_CLASSES, [255, '*']]);

Object.freeze(DNS_CLASSES);
Object.freeze(DNS_CLASSES);

export { DNS_CLASSES, DNS_QCLASSES };
