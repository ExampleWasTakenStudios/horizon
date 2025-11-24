const DNS_CLASSES: ReadonlyMap<number, string> = new Map<
  number,
  string
>([
  [1, 'IN'] as const,
  [3, 'CH'] as const,
  [4, 'HS'] as const,
]);

const DNS_QCLASSES = new Map<number, string>([...DNS_CLASSES, [255, '*']]);

Object.freeze(DNS_CLASSES);
Object.freeze(DNS_CLASSES);

export { DNS_CLASSES, DNS_QCLASSES };
