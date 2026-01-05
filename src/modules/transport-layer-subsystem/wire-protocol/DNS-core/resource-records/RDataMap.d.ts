import type { DNS_TYPES } from '../constants/DNS_TYPES.ts';

export interface RDataMap {
  [DNS_TYPES.A]: string;
  [DNS_TYPES.NS]: string;
  [DNS_TYPES.CNAME]: string;
  [DNS_TYPES.SOA]: SOA_Type;
  [DNS_TYPES.PTR]: string;
  [DNS_TYPES.HINFO]: HINFO_Type;
  [DNS_TYPES.TXT]: string[];
  [DNS_TYPES.MX]: MX_Type;
  /**
   * We are not supporting any RDATA format at the moment - hence this is unknown and we simply skip RDLENGTH bytes.
   */
  [DNS_TYPES.OPT]: unknown;
}

export interface SOA_Type {
  mName: string;
  rName: string;
  serial: number;
  refresh: number;
  retry: number;
  expire: number;
  minimum: number;
}

export interface HINFO_Type {
  cpu: string;
  os: string;
}

export interface MX_Type {
  preference: number;
  exchange: string;
}
