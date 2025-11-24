const DNS_TYPES: ReadonlyMap<number, string> = new Map<number, string>([
  [1, 'A'],
  [2, 'NS'],
  [5, 'CNAME'],
  [11, 'WKS'],
  [12, 'PTR'],
  [13, 'HINFO'],
  [14, 'MINFO'],
  [15, 'MX'],
  [16, 'TXT'],
]);

const DNS_QTYPES: ReadonlyMap<number, string> = new Map<number, string>([
  ...DNS_TYPES,
  [252, 'AXFR'],
  [253, 'MAILB'],
  [254, 'MAILA'],
  [255, '*'],
]);

Object.freeze(DNS_TYPES);
Object.freeze(DNS_QTYPES);

export { DNS_TYPES, DNS_QTYPES };
