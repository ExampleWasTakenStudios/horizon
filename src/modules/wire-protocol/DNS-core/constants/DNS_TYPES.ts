export enum DNS_TYPES {
  A = 1,
  NS = 2,
  CNAME = 5,
  WKS = 11,
  PTR = 12,
  HINFO = 13,
  MINFO = 14,
  MX = 15,
  TXT = 16,
}

export enum DNS_QTYPES {
  A = 1,
  NS = 2,
  CNAME = 5,
  WKS = 11,
  PTR = 12,
  HINFO = 13,
  MINFO = 14,
  MX = 15,
  TXT = 16,
  AXFR = 252,
  MAILB = 253,
  MAILA = 254,
  '*' = 255,
}
